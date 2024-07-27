struct Estudiante {
    nombre: String,
    puntaje_acumulado: i32,
    asistencia: i32,
    puntos_apreciacion: i32,
}

fn nuevo(nombre: &str, puntaje_acumulado: i32, asistencia: i32, puntos_apreciacion: i32) -> Estudiante {
    Estudiante {
        nombre: nombre.to_string(),
        puntaje_acumulado,
        asistencia,
        puntos_apreciacion,
    }
}

fn calcular_nota_definitiva(estudiante: &Estudiante) -> i32 {
    let mut nota_definitiva = estudiante.puntaje_acumulado;
    if estudiante.asistencia < 75 {
        nota_definitiva -= 1;
    }
    if estudiante.asistencia < 50 {
        nota_definitiva -= 2;
    }
    if estudiante.asistencia < 25 {
        nota_definitiva -= 3;
    }
    nota_definitiva += estudiante.puntos_apreciacion;
    nota_definitiva
}

fn mostrar(estudiante: &Estudiante) {
    println!("{}: nota definitiva {}", estudiante.nombre, calcular_nota_definitiva(estudiante));
}

fn main() {
    let estudiante = nuevo("Maria Gonzalez", 9, 100, 1);
    mostrar(&estudiante);
}

