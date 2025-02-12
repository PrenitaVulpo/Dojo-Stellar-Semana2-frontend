use colored::*;
fn main() {
    let team_logo: &str = r#"
      _________  __           .__   .__                     _________                                .__         
 /   _____/_/  |_   ____  |  |  |  |  _____  _______   /   _____/  ____    ____    ______  ____  |__|  ______
 \_____  \ \   __\_/ __ \ |  |  |  |  \__  \ \_  __ \  \_____  \ _/ __ \  /    \  /  ___/_/ __ \ |  | /  ___/
 /        \ |  |  \  ___/ |  |__|  |__ / __ \_|  | \/  /        \\  ___/ |   |  \ \___ \ \  ___/ |  | \___ \ 
/_______  / |__|   \___  >|____/|____/(____  /|__|    /_______  / \___  >|___|  //____  > \___  >|__|/____  >
        \/             \/                  \/                 \/      \/      \/      \/      \/          \/ 
    "#;
    let menu_header: &str = "======================================ESCOLHA UMA DAS OPÇÕES ABAIXO===========================================";
    let menu_footer: &str = "==============================================================================================================";
    println!("{}", team_logo.green());
    println!("{}", menu_header.blue());
    println!("{}", "1 - Criar par de chaves".green());
    println!("{}", "2 - Listar pares disponíveis".green());
    println!("{}", "3 - Buscar Saldo da Carteira".green());
    println!("{}", "4 - Enviar XML para outros Endereços".green());
    println!("{}", "Para sair da aplicação, use CTRL + C".red());
    println!("{}", menu_footer.blue());
}
