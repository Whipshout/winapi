#[derive(Copy, Clone)]
pub struct Logger;

impl Logger {
    pub fn info(&self, msg: &str) {
        println!("{msg}");
    }

    pub fn separator(&self) {
        println!("-----------------------------------------------------------------------");
    }

    pub fn welcome(&self) {
        self.separator();
        self.info(
            r"NO HACE FALTA TENER LA PANTALLA DEL LOST ARK EN PRIMER PLANO.
PUEDE ESTAR MINIMIZADO Y FUNCIONA IGUAL.

HAY QUE TENER LOS SIGUIENTES KEYBINDINGS EN EL JUEGO:
            
-- MONTURA -------------------------------------------- *** 5 ***
-- CANCION PARA STRONGHOLD (Song of Hearth and Home) -- *** 6 ***
-- CANCION PARA SALIR (Song of Escape)----------------- *** F1 ***

EN CASO DE NO TENERLOS ASI, HAY QUE CAMBIARLOS Y VOLVER A ABRIR ESTO",
        );
        self.separator();
        self.info("EMPIEZA LA ACCION !!!");
        self.separator();

        self.info("RECUPERANDO EL PROCESO DEL LOST ARK...");
        self.separator();
    }

    pub fn exit(&self) {
        self.info("ALGO SALIO MAL, CERRANDO");
        self.separator();
    }
}
