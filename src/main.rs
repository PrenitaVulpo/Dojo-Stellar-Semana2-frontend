use colored::*;
use std::io;

fn main() {
    menu_main();
    // println!("{}", "1 - Criar par de chaves".green());
    // println!("{}", "2 - Listar pares disponíveis".green());
    // println!("{}", "3 - Buscar Saldo da Carteira".green());
    // println!("{}", "4 - Enviar XML para outros Endereços".green());
    // println!("{}", "Para sair da aplicação, use CTRL + C".red());
}

fn load_menu_header() {
    let team_logo: &str = r#"
    _________  __           .__   .__                     _________                                .__         
/   _____/_/  |_   ____  |  |  |  |  _____  _______   /   _____/  ____    ____    ______  ____  |__|  ______
\_____  \ \   __\_/ __ \ |  |  |  |  \__  \ \_  __ \  \_____  \ _/ __ \  /    \  /  ___/_/ __ \ |  | /  ___/
/        \ |  |  \  ___/ |  |__|  |__ / __ \_|  | \/  /        \\  ___/ |   |  \ \___ \ \  ___/ |  | \___ \ 
/_______  / |__|   \___  >|____/|____/(____  /|__|    /_______  / \___  >|___|  //____  > \___  >|__|/____  >
      \/             \/                  \/                 \/      \/      \/      \/      \/          \/ 
  "#;
    let menu_header: &str = "======================================ESCOLHA UMA DAS OPÇÕES ABAIXO===========================================";
    println!("{}", team_logo.green());
    println!("{}", menu_header.blue());
}

fn load_menu_footer() {
    let menu_footer: &str = "==============================================================================================================";
    println!("{}", menu_footer.blue());
}

fn menu_main() {
    let mut option: String = String::new();

    loop {
        load_menu_header();
        println!("{}", "1 - Conectar Carteira".green());
        println!(
            "{}",
            "2 - Acessar menu de opções (requer carteira conectada)".green()
        );
        println!("{}", "3 - Verificar conecção com a carteira".green());
        println!("{}", "4 - Sair da aplicação".red());
        load_menu_footer();

        //input
        println!("{}", "Digite a opção desejada (somente números):".yellow());
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Entrada inválida! Use somente números.".red());
                continue;
            }
        };
        match option {
            //integrar a função de conectar a carteira no lugar disso aí
            1 => println!("One"),
            2 => println!("Two"),
            //integrar a função de ver se existe uma carteira conectada, basta imprimir se
            //a carteira está conectada ou não e voltar aqui 
            3 => println!("Three"),
            4 => {
                println!("saindo");
                break;
            }
            _ => println!("{}", "erro, tente novamente".red()),
        }
    }
}

fn menu_post_wallet_load() {
    let mut option: String = String::new();

    loop {
        load_menu_header();
        println!("{}", "1 - Criar par de chaves".green());
    println!("{}", "2 - Listar pares disponíveis".green());
    println!("{}", "3 - Buscar Saldo da Carteira".green());
    println!("{}", "4 - Enviar XML para outros Endereços".green());
    println!("{}", "Para sair da aplicação, use CTRL + C".red());
        load_menu_footer();
    }
};
