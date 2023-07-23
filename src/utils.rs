
pub enum Path {
    OptionButton,
    PlayButton,
    QuitButton,
    ResumeButton,
    Font,

}

pub fn get_paths(path: Path) -> String {
    let exe_path = std::env::current_exe().unwrap();

    match path {
        Path::OptionButton => {
            let resource_path = exe_path.parent().unwrap().join("sprites/Options-Button.png");
            return resource_path.to_str().unwrap().to_string();
        }
        Path::PlayButton => {
            let resource_path = exe_path.parent().unwrap().join("sprites/Play-Button.png");
            return resource_path.to_str().unwrap().to_string();
        }
        Path::QuitButton => {
            let resource_path = exe_path.parent().unwrap().join("sprites/Quit-Button.png");
            return resource_path.to_str().unwrap().to_string();
        }
        Path::ResumeButton => {
            let resource_path = exe_path.parent().unwrap().join("sprites/Resume-Button.png");
            return resource_path.to_str().unwrap().to_string();
        }
        Path::Font => {
            let resource_path = exe_path.parent().unwrap().join("fonts/Righteous-Regular.ttf");
            return resource_path.to_str().unwrap().to_string();
        }
    }
}