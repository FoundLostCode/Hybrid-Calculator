use std::io;
use std::f64;
fn main() {
    let mut pause = String::new();

    println!("This is a hybrid computer whose existence is only justified by the fact that I can and want to learn it.");
    print!("Enter the number of the calculation operation:\n");
    loop {
        let mut wahl = String::new();
        println!("For people who often mistype and use unusual calculation methods,\n\
        enter the corresponding number for a desired arithmetic operation.\n");
        println!("[1]Additon of n numbers\n[2]Check whether the number is even\n[3]Check whether the number is divisible by n\n[4]Calculate factorial of a number\n[5]Reverse a number (useful if the number is very large, then copy it)\n\
        [6]Is the number a prime number\n[7]Convert number to binary representation\n[8]Convert decimal to hexadecimal\n[9]Calculate length/amount of a vector\n[10]Addition or subtraction of n vectors\n[11]Connection vector\n[12]Scalar * Vector (Vector * S)\n\
        [13]Scalar product of two vectors\n[14]Pythagorean theorem (a^2 +b^2=c^2)\n[15]Display series\n[16]Fibonacci\n");
        io::stdin().read_line(&mut wahl).expect("Zahl nicht zulässig\n");
        let ergeingabe: i32 = wahl.trim().parse().expect("Fehler bei umwandlung\n");

        #[allow(clippy::match_same_arms)]
        match ergeingabe {
            1=> {
                println!("Geben Sie die anzahl der Zahlen ein\n");
                let mut anzahl_arry = String::new();
                io::stdin().read_line(&mut anzahl_arry).expect("Fehler bei eingabe\n");
                let n = anzahl_arry.trim().parse().expect("Fehler bei eingabe");
                let mut sume = 0;
                //for schleife
                for _ in 0..n {
                    let mut eingabe_zahlen = String::new();
                    println!("Geben sie die Zahlen ein");
                    io::stdin().read_line(&mut eingabe_zahlen).expect("Fehler bei eingabe\n");
                    //wieder chasten
                    let endgutig: i32 = eingabe_zahlen.trim().parse().expect("Fehler bei eingabe\n");
                    sume += endgutig;
                }
                println!("Die Summe deiner Eingaben entspricht: {}", sume);
                io::stdin().read_line(&mut pause).unwrap();
            }
            2=>{ println!("Geben die die Zahl ein die Auf gerade oder nicht geprüft werden soll:\n");
                let mut gerade = String::new();
                io::stdin().read_line(&mut gerade).expect("Fehler beim lesen der zahl\n");
                let zahl_gerade: i32 = gerade.trim().parse().expect("Fehler bei umwandlung\n");
                if zahl_gerade % 2 == 0 {
                    println!("{} IST gerade\n", zahl_gerade);
                    io::stdin().read_line(&mut pause).unwrap();
                } else {
                    println!("{} ist NICHT gerade\n", zahl_gerade);
                    io::stdin().read_line(&mut pause).unwrap();
                }
            }
            3=>{
                println!("Geben sie erst die Zahl und dan ein wodurch geprüft werden soll ob sie Teilbar ist:\n");
                let mut zahl_ob = String::new();
                let mut zahl_n = String::new();
                io::stdin().read_line(&mut zahl_ob).expect("Fehler bei umwandlung von zahl_ob\n");
                let zahl_ob_new: i32 = zahl_ob.trim().parse().expect("Fehler bei umwandlung von zahl_ob\n");
                io::stdin().read_line(&mut zahl_n).expect("Fehler bei umwandlung von zahl_n\n");
                let zahl_n_new: i32 = zahl_n.trim().parse().expect("Fehler bei umwandlung von zahl_n_new\n");

                if zahl_ob_new % zahl_n_new == 0 {
                    println!("{} ist durch {} teilbar!", zahl_ob_new, zahl_n_new);
                    io::stdin().read_line(&mut pause).unwrap();
                } else {
                    println!("{} ist NICHT durch {} teilbar", zahl_ob_new, zahl_n_new);
                    io::stdin().read_line(&mut pause).unwrap();
                }
            }
            4=>{
                println!("Geben sie die Zahl ein deren Fakultät berechnet werden soll (Max: 34!)\n");
                let mut fakul = String::new();
                io::stdin().read_line(&mut fakul).expect("Fehler bei eingabe\n");
                //Nach höheren Datentyp als u128 schauen. Max: 34! machbar.
                let faku_rg: u128 = fakul.trim().parse().expect("Fehler bei eingabe\n");

                let mut zahler = 1;
                for i in 1..=faku_rg {
                    zahler *= i;
                }
                println!("Die Fakultät von {}ist {}\n", fakul, zahler);
                io::stdin().read_line(&mut pause).unwrap();
            }
            5=>{
                println!("Geben Sie die Zahl ein die Sie Umkehren wollen.\n");
                let mut dreher = String::new();
                io::stdin().read_line(&mut dreher).expect("Fehler bei eingabe\n");
                let mut der_dreher: u128 = dreher.trim().parse().expect("Fehler bei eingabe\n");
                //jetzt die Zahl umdrehen
                let mut muf = 0;

                while der_dreher > 0 {
                    muf = muf * 10 + der_dreher % 10;
                    der_dreher /= 10;
                }
                println!("Die Umgekehrte Zahl:{}\n", muf);
                io::stdin().read_line(&mut pause).unwrap();
            }
            6=>{
                println!("Geben Sie die Zahl ein die auf eine Primzahl untersucht werden soll:\n");
                let mut prim = String::new();
                io::stdin().read_line(&mut prim).expect("Fehler bei eingabe\n");
                let prim_mew: i32 = prim.trim().parse().expect("Fehler bei eingabe");
                if prim_mew == 2 {
                    println!("Ist eine Primzahl!\n");
                    io::stdin().read_line(&mut pause).unwrap();
                }
                if prim_mew <= 1 {
                    println!("Keine Primzahl!\n");
                    io::stdin().read_line(&mut pause).unwrap();
                }

                let sqrt_n: i32 = (prim_mew as f64).sqrt() as i32;
                let mut is_prime = true;
                for i in (3..=sqrt_n).step_by(2) {
                    if prim_mew % i == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    println!("Die Zahl ist eine Primzahl!");
                    io::stdin().read_line(&mut pause).unwrap();
                } else {
                    println!("Keine Primzahl!");
                    io::stdin().read_line(&mut pause).unwrap();
                }
            }
            7=>{
                println!("Geben Sie die Zahl ein die Sie in Binär umrechnen wollen:\n");
                let mut bin_umrechnent = String::new();
                io::stdin().read_line(&mut bin_umrechnent).expect("Fehler bei eingabe\n");
                let die_bin_zahl:u128=bin_umrechnent.trim().parse().expect("Fehler bei eingabe");
                //Vector ist ein Dynamische Array
                let mut array =Vec::new();
                let mut copy = die_bin_zahl;

                while copy>0{
                    array.push(copy%2);
                    copy /=2;
                }
                array.reverse();
                println!("Binärform:{:?}",array);
                println!("Ohne [] und , natürlich\n");
                io::stdin().read_line(&mut pause).unwrap();
            }
            8=>{
                println!("Geben Sie eine Zahl ein die in Hexerdezimal umgewandelt werden soll:\n");
                let mut umw = String::new();
                io::stdin().read_line(&mut umw).expect("Fehler bei eingabe\n");
                let mut cast_wmw: u128 = umw.trim().parse().expect("Fehler bei eingabe");
                //vekor für die reste zum spechern
                let mut reste = Vec::new();

                while cast_wmw > 0 {
                    reste.push(cast_wmw % 16);
                    cast_wmw /= 16;
                }
                println!("Die einzelnen Ziffern sind durch einen Leerzeichen getrennt.\nDie zahlen 10-15 müssen durch die Buchstaben A-F ersetzt werden.\nA=10 B=11 C=12\nD=13 E=14 F=15\n");
                reste.reverse();
                for i in &reste {
                    print!("{} ", i);
                }
                println!();
                io::stdin().read_line(&mut pause).unwrap();
            }
            9=>{
                println!("Geben Sie erst die anzahl der Dementionen des Vektors ein:\n");
                let mut demension=String::new();
                io::stdin().read_line(&mut demension).expect("Fehler bei eingabe\n");
                let fest_demension: usize =demension.trim().parse().expect("Fehler bei eingabe");
                let mut zahlen:Vec<f64>=Vec::new();

                for _ in 0..fest_demension {
                    let mut xyz=String::new();
                    println!("Geben Sie die Zahlen ein:\n");
                    io::stdin().read_line(&mut xyz).expect("Fehler bei eingabe\n");
                    let xyz_end:f64=xyz.trim().parse().expect("Fehler bei eingabe");
                    //Datentyp casting weil in Vector
                    zahlen.push(xyz_end);
                }
                let mut endergebn:f64=0.0;
                for zahl in &zahlen {
                    endergebn+=zahl* zahl;
                }
                let betrag=endergebn.sqrt();
                println!("Der Betrag des Vektors ist: {:.2}\n",betrag);
                io::stdin().read_line(&mut pause).unwrap();
            }
            10=>{
                println!("Geben Sie ein ob sie Add.(+) oder Subtrac.(-) wollen:\n");
                let mut operator=String::new();
                io::stdin().read_line(&mut operator).expect("Fehler bei eingabe\n");
                let operator_check:String=operator.trim().parse().expect("Fehler bei eingabe");

                if operator_check== "-" {
                    println!("Subtraktion gewählt\n");
                    println!("Geben Sie die Anzahl der Vektoren ein:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Fehler bei Eingabe");
                    let anzahl_vectoren: usize = input.trim().parse().expect("Fehler bei Eingabe");

                    println!("Geben Sie die Dimension der Vektoren ein:");
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    let dimension_new: usize = input.trim().parse().expect("Fehler bei Eingabe");

                    let mut vectoren: Vec<Vec<f64>> = Vec::new();
                    for vector_index in 0..anzahl_vectoren {
                        println!("Geben Sie die Werte für Vektor {} ein:", vector_index + 1);
                        let mut vektor: Vec<f64> = Vec::new();
                        for i in 0..dimension_new {
                            let mut wert_input = String::new();
                            println!("Geben Sie den {}. Dimensionswert an:", i + 1);
                            io::stdin().read_line(&mut wert_input).expect("Fehler bei Eingabe");
                            let wert: f64 = wert_input.trim().parse().expect("Fehler bei Eingabe");
                            vektor.push(wert);
                        }
                        vectoren.push(vektor);
                    }
                    if vectoren.is_empty() {
                        println!("Es wurde kein Vektor eingegeben.");
                        return;
                    }
                    let mut ergebnis_vec = vectoren[0].clone();

                    for vektor in &vectoren[1..] {
                        for i in 0..dimension_new {
                            ergebnis_vec[i] -= vektor[i];
                        }
                    }
                    println!("Der Ergebnisvektor ist: {:?}", ergebnis_vec);
                }
                else if operator_check!= "+" && operator_check!= "-"{
                    println!("Ungültige Operator Eingabe\n");
                }
                else if operator_check== "+"{
                    println!("Addition gewählt\n");
                    println!("Geben Sie die Anzahl der Vektoren ein:");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Fehler bei Eingabe");
                    let anzahl_vectoren: usize = input.trim().parse().expect("Fehler bei Eingabe");
                    println!("Geben Sie die Dimension der Vektoren ein:");
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    let demension_new: usize = input.trim().parse().expect("Fehler bei Eingabe");
                    let mut vectoren: Vec<Vec<f64>> = Vec::new();
                    for vector_index in 0..anzahl_vectoren {
                        println!("Geben Sie die Werte für Vektor {} ein:", vector_index + 1);
                        let mut vektor: Vec<f64> = Vec::new();
                        for i in 0..demension_new {
                            let mut wert_input = String::new();
                            println!("Geben Sie den {}. Dimensionswert an:", i + 1);
                            io::stdin().read_line(&mut wert_input).expect("Fehler bei Eingabe");
                            let wert: f64 = wert_input.trim().parse().expect("Fehler bei Eingabe");
                            vektor.push(wert);
                        }
                        vectoren.push(vektor);
                    }
                    let mut ergebnis_vec = vec![0.0; demension_new];
                    for vektor in &vectoren {
                        for i in 0..demension_new {
                            ergebnis_vec[i] += vektor[i];
                        }
                    }
                    println!("Der Ergebnisvektor ist: {:?}", ergebnis_vec);
                }
                else if operator_check!= "+" && operator_check!= "-"{
                    println!("Ungültige Operator Eingabe\n");
                }
            }
            11=>{
                println!("Geben Sie die Dimension beider Vektoren an deren Verbindunsvektor Sie bestimmen möchten:\n");
                let mut dimsension_verb=String::new();
                io::stdin().read_line(&mut dimsension_verb).expect("Fehler bei Eingabe");
                let dimension_demension:usize=dimsension_verb.trim().parse().expect("Fehler bei Eingabe");
                let mut vec_eins:Vec<f64> = Vec::new();
                let mut vec_zwei:Vec<f64> = Vec::new();
                println!("Gebe die Zahlen des ersten Vectors ein:\n");
                for _ in 0..dimension_demension{
                    let mut eingabe=String::new();
                    io::stdin().read_line(&mut eingabe).expect("Fehler bei einlesen\n");
                    let zwichen=eingabe.trim().parse().expect("Fehler bei Eingabe");
                    vec_eins.push(zwichen);
                }
                println!("Geben SIe die Zahlen des zweiten Vectors an:\n");
                for _ in 0..dimension_demension{
                    let mut eingabe_new=String::new();
                    io::stdin().read_line( &mut eingabe_new).expect("Fehler bei Eingabe");
                    let zwichen_new=eingabe_new.trim().parse().expect("Fehler bei Eingabe");
                    vec_zwei.push(zwichen_new);
                }
                println!();
                let mut vec_final=Vec::new();
                for i in 0..dimension_demension{
                    let ergebn= vec_zwei[i]-vec_eins[i];
                    vec_final.push(ergebn);
                }
                println!("Dein Verbingngsvector hat die Koordinaten:\n");
                for i in 0..dimension_demension{
                    println!("{}",vec_final[i]);
                }
                io::stdin().read_line(&mut pause).expect("Fehler bei Eingabe");

            }
            12=>{
                println!("Geben Sie den Vektor sowie die zahl ein:\n");
                let mut s:String= String::new();
                let mut vec: Vec<i32> = Vec::new();
                println!("Geben Sie die Zahl S ein:\n");
                io::stdin().read_line(&mut s).expect("Fehler bei eingabe von S");
                let s_new:i32=s.trim().parse().expect("Fehler beim kompriniernen in let s_new");
                println!("Geben Sie die Dimension des Vektors an:\n");
                let mut dimension_s=String::new();
                io::stdin().read_line(&mut dimension_s).expect("Fehler bei Eingabe");
                let new_demension_s:usize =dimension_s.trim().parse().expect("Fehler bei Eingabe");

                let mut eingabe_skalar:String= String::new();
                for _ in 0..new_demension_s{
                    eprintln!("Geben Sie die zahlen des vektors an.\n");
                    io::stdin().read_line(&mut eingabe_skalar).expect("Fehler bei Eingabe");
                    let new_array:i32= eingabe_skalar.trim().parse().expect("Fehler bei Eingabe");
                    vec.push(new_array * s_new);
                    eingabe_skalar.clear();
                }
                //Ausgabe
                let mut edger=0;
                for i in 0..new_demension_s {
                    edger += vec[i];
                }
                println!("Ihr sakalrproduckt {}\n",edger);
            }
            13=>{
                let mut dimension:String=String::new();
                let mut zahlen_scale:String= String::new();
                let mut vec_der_salare:Vec<f64>=Vec::new();
                let mut vec_der_salare2:Vec<f64>=Vec::new();
                //
                println!("Geben Sie die Dimension der Vectoren ein:\n");
                io::stdin().read_line(&mut dimension).expect("Fehler bei Eingabe");
                let dimension_fest=dimension.trim().parse().expect("Fehler bei Eingabe");
                //
                println!("Geben Sie die Zahlen des ersten Vectors ein:\n");//Erster Vector
                for _ in 0..dimension_fest{
                    println!("Zahl bitte:\n");
                    io::stdin().read_line(&mut zahlen_scale).expect("Fehler bei Eingabe");
                    let fest_zaglen_scale:f64=zahlen_scale.trim().parse().expect("Fehler bei Eingabe");
                    vec_der_salare.push(fest_zaglen_scale);
                    zahlen_scale.clear();
                }
                //
                eprintln!("Geben Sie die Zahlen des zweiten Vectors ein:\n");
                for _ in 0..dimension_fest{
                    println!("Zahl bitte:\n");
                    io::stdin().read_line(&mut zahlen_scale).expect("Fehler bei Eingabe");
                    let fest_zaglen_scale2:f64=zahlen_scale.trim().parse().expect("Fehler bei Eingabe");
                    vec_der_salare2.push(fest_zaglen_scale2);
                    zahlen_scale.clear();
                }
                //
                let mut ergebniss_scalar:f64=0.0;
                for i in 0..dimension_fest{
                    ergebniss_scalar+= vec_der_salare[i]*vec_der_salare2[i];
                }
                println!("Ihr Scalarproduckt ist: {}\n",ergebniss_scalar);

            }
            14=>{
                let mut a_qua:String=String::new();
                let mut b_qua:String=String::new();
                println!("Geben Sie zuerst die Zahl für a und dan für b ein:\n");
                //
                io::stdin().read_line(&mut a_qua).expect("Fehler bei Eing");
                let a_new:u128=a_qua.trim().parse().expect("Fehler bei Eing");
                io::stdin().read_line(&mut b_qua).expect("Fehler bei Eing");
                let b_new:u128=b_qua.trim().parse().expect("Fehler bei Eing");
                //
                let c_qua= (a_new*a_new) + (b_new*b_new);
                println!("C^2: {}\n",c_qua);
                println!("Willste C ziehste die Wurzel\n");

            }
            15=> {
                println!("Gib zahl für Reihe ein:\n");
                let mut speicher=0.0;
                let mut shit_number: String = String::new();
                io::stdin().read_line(&mut shit_number).expect("Fehler bei Eing");
                let shit_shit_new:f64= shit_number.trim().parse().expect("Fehler bei Umw");
                //
                println!("Gib ein bis zur Stelle:\n");
                let mut fuck_shit: String = String::new();
                io::stdin().read_line(&mut fuck_shit).expect("Fehler bei Eing");
                println!();
                let fuck_fuck_shit = fuck_shit.trim().parse().expect("Fehler bei Umw");
                //
                for _ in 0..fuck_fuck_shit {
                    speicher +=shit_shit_new;
                    println!("{}\n", speicher);
                }
            }

            _ => {
                let skull = r#"
         ______
      .-'      `-.
     /            \
    |              |
    |,  .-.  .-.  ,|
    | )(_o/  \o_)( |
    |/     /\     \|
    (_     ^^     _)
     \__|IIIIII|__/
      | \IIIIII/ |
      \          /
       `--------`
    "#;
                println!("{}", skull);
                io::stdin().read_line(&mut pause).unwrap();
            }
        }
    }

}

