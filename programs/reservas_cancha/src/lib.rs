use anchor_lang::prelude::*;

declare_id!("EZ5TJT71SDcrumYrxtcMMNEmayuqqjShn5CK5sauQiwt"); // Al hacer Build, se actualizará solo

#[program]
pub mod reservas_cancha {
    use super::*;

    // crear la cancha
    pub fn crear_cancha(ctx: Context<NuevaCancha>, nombre: String) -> Result<()> {
        let cancha = &mut ctx.accounts.cancha;
        cancha.owner = ctx.accounts.owner.key();
        cancha.nombre = nombre;
        cancha.usuarios = Vec::new(); // Corregido: usuarios en lugar de reservas
        cancha.id_counter = 0;
        Ok(())
    }

    // hacer la reserva
    pub fn hacer_reserva(
        ctx: Context<ModificarCancha>,
        id_usuario: u16,
        horario: String,
        monto: u32,
    ) -> Result<()> {
        require!(
            ctx.accounts.cancha.owner == ctx.accounts.owner.key(),
            Errores::NoOwner
        );

        let cancha = &mut ctx.accounts.cancha;
        let nueva_reserva = Reserva { horario: horario.clone(), monto, cancelada: false };

        if id_usuario == 0 {
            // Usuario nuevo
            cancha.id_counter += 1;
            let id = cancha.id_counter;
            cancha.usuarios.push(Usuario {
                id,
                reservas_activas: 1,
                historial: vec![nueva_reserva],
                cancelaciones: 0,
            });
            Ok(())
        } else {
            // Usuario existente
            for i in 0..cancha.usuarios.len() {
                if cancha.usuarios[i].id == id_usuario {
                    
                    require!(
                        cancha.usuarios[i].reservas_activas < 3,
                        Errores::LimiteReservas
                    );
                    
                    // MEJORA 3: Validación de Horario
                    for reserva in &cancha.usuarios[i].historial {
                        if reserva.horario == horario && !reserva.cancelada {
                            return Err(Errores::HorarioOcupado.into());
                        }
                    }

                    cancha.usuarios[i].historial.push(nueva_reserva);
                    cancha.usuarios[i].reservas_activas += 1;
                    return Ok(());
                }
            }
           
            Err(Errores::NoUser.into())
        }
    }

    // cancelar la reserva
    pub fn cancelar_reserva(
        ctx: Context<ModificarCancha>,
        id_usuario: u16,
        horario: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.cancha.owner == ctx.accounts.owner.key(),
            Errores::NoOwner
        );
        
        let cancha = &mut ctx.accounts.cancha;
        for i in 0..cancha.usuarios.len() {
            if cancha.usuarios[i].id == id_usuario {
                for j in 0..cancha.usuarios[i].historial.len() {
                    if cancha.usuarios[i].historial[j].horario == horario && !cancha.usuarios[i].historial[j].cancelada {
                        
                        cancha.usuarios[i].historial[j].cancelada = true;
                        cancha.usuarios[i].reservas_activas -= 1;
                        cancha.usuarios[i].cancelaciones += 1;
                        
                        if cancha.usuarios[i].cancelaciones >= 3 {
                             msg!("Usuario bloqueado por exceso de cancelaciones");
                        }
                        return Ok(());
                    }
                }
            }
        }
        Err(Errores::NoUser.into())
    }
}


#[account]
#[derive(InitSpace)]
pub struct Cancha {
    pub owner: Pubkey,
    #[max_len(30)]
    pub nombre: String,
    #[max_len(10)]
    pub usuarios: Vec<Usuario>,
    pub id_counter: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Usuario {
    pub id: u16,
    pub reservas_activas: u16, 
    pub cancelaciones: u16,    
    #[max_len(10)]
    pub historial: Vec<Reserva>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Reserva {
    #[max_len(20)]
    pub horario: String, 
    pub monto: u32,
    pub cancelada: bool, 
}


#[error_code]
pub enum Errores {
    #[msg("No eres el owner")]
    NoOwner,
    #[msg("Usuario no existe")]
    NoUser,
    #[msg("Límite de 3 reservas alcanzado")]
    LimiteReservas,
    #[msg("Ese horario ya está ocupado")]
    HorarioOcupado,
}


#[derive(Accounts)]
pub struct NuevaCancha<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Cancha::INIT_SPACE + 8,
        seeds = [b"cancha", owner.key().as_ref()],
        bump
    )]
    pub cancha: Account<'info, Cancha>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarCancha<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub cancha: Account<'info, Cancha>,
}
