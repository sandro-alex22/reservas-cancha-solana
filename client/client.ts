import { PublicKey } from "@solana/web3.js";


function pdaCancha() {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("cancha"), // Debe coincidir con la semilla en Rust
      pg.wallet.publicKey.toBuffer(),
    ],
    pg.PROGRAM_ID
  );
}

// crear la cancha
async function crearCancha(nombre: string) {
  const [pda_cancha] = pdaCancha();
  const txHash = await pg.program.methods
    .crearCancha(nombre)
    .accounts({
      owner: pg.wallet.publicKey,
      cancha: pda_cancha,
    })
    .rpc();
  console.log("Cancha creada! TxHash: ", txHash);
}

// hacer una reserva
async function hacerReserva(id_usuario: number, horario: string, monto: number) {
  const [pda_cancha] = pdaCancha();
  try {
    const txHash = await pg.program.methods
      .hacerReserva(id_usuario, horario, monto)
      .accounts({
        owner: pg.wallet.publicKey,
        cancha: pda_cancha,
      })
      .rpc();
    console.log("Reserva exitosa! TxHash: ", txHash);
  } catch (error) {
    console.error("Error al reservar (Quizás límite alcanzado o horario ocupado):", error);
  }
}

// cancelar
async function cancelarReserva(id_usuario: number, horario: string) {
  const [pda_cancha] = pdaCancha();
  const txHash = await pg.program.methods
    .cancelarReserva(id_usuario, horario)
    .accounts({
      owner: pg.wallet.publicKey,
      cancha: pda_cancha,
    })
    .rpc();
  console.log("Reserva cancelada! TxHash: ", txHash);
}

// ver el estado de la cuenta
async function verCancha() {
  const [pda_cancha] = pdaCancha();
  try {
    const canchaAccount = await pg.program.account.cancha.fetch(pda_cancha);
    console.log("Nombre de la cancha:", canchaAccount.nombre);
    console.log("Cantidad de usuarios:", canchaAccount.usuarios.length);
    
    for (let i = 0; i < canchaAccount.usuarios.length; i++) {
        const user = canchaAccount.usuarios[i];
        console.log(`Usuario ID: ${user.id} | Reservas Activas: ${user.reservasActivas} | Cancelaciones: ${user.cancelaciones}`);
        console.log("Historial:", user.historial);
    }
  } catch (error) {
    console.error("Error leyendo la cancha:", error);
  }
}

// PRUEBAS
//crearCancha("Cancha Central");
//hacerReserva(0, "2026-11-25 10:00", 50);
// hacerReserva(1, "2026-11-25 10:00", 50); 
//hacerReserva(1, "2026-11-25 11:00", 50);
//hacerReserva(1, "2026-11-25 12:00", 50);
 //hacerReserva(1, "2026-11-25 13:00", 50); 
//cancelarReserva(1, "2026-11-25 10:00");
 //verCancha();
