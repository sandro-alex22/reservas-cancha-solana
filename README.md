
# Proyecto Final: Sistema de Reservas de Canchas (Solana)

**Alumno:** Alexsandro Vazquez Salvador
**Bootcamp:** Advanced - Solana Engineering Track

## 1. Descripción del Proyecto
Sistema descentralizado para gestionar reservas de canchas deportivas. Permite crear una cancha y registrar reservas. Construido con Rust y Anchor, desplegado en Devnet.

## 2. Program ID (Devnet)
El programa está desplegado en Devnet con el ID:
("EZ5TJT71SDcrumYrxtcMMNEmayuqqjShn5CK5sauQiwt")

## 3. Aporte Propio (Mejoras Implementadas)
Se añadieron 3 reglas de negocio:
1.  **Límite de reservas:** Un usuario no puede tener más de 3 reservas activas.
2.  **Penalización por cancelación:** La reserva no se borra, se marca como cancelada y suma al contador del usuario.
3.  **Validación de horarios:** El programa rechaza un horario si ya está reservado.

## 4. Evidencias de Funcionamiento

**A. Creación de la Cancha:**
<img width="980" height="467" alt="image" src="https://github.com/user-attachments/assets/080d4dad-cc95-405a-ab46-feac9acf8283" />


**B. Reserva Exitosa:**
<img width="980" height="468" alt="image" src="https://github.com/user-attachments/assets/b796a4af-d0e3-4c5d-99d5-cae8ed3d607a" />


**C. Error por Horario Ocupado (Mejora 3):**
<img width="980" height="468" alt="image" src="https://github.com/user-attachments/assets/2e88f475-68f0-4055-a06f-fc78d336e188" />


**D. Error por Límite de Reservas (Mejora 1):**
<img width="980" height="465" alt="image" src="https://github.com/user-attachments/assets/e0e7abbe-bbdb-444b-b8c0-c5f2b545ee73" />


**E. Cancelación Exitosa (Mejora 2):**
<img width="980" height="468" alt="image" src="https://github.com/user-attachments/assets/1274db38-2b89-497d-b9f2-dc633eee222b" />


**F. Estado final (verCancha):**
<img width="980" height="469" alt="image" src="https://github.com/user-attachments/assets/e0a905d3-7be7-460f-9f30-83598c45b65d" />
