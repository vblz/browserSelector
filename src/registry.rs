use std::io;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;
use std::error::Error;

fn register(exe_res: io::Result<PathBuf>, ico_res: io::Result<PathBuf>) -> Result<(), Error> {
    if exe_res.is_err() {
        return Err(exe_res.unwrap_err());
    }

    if ico_res.is_err() {
        return Err(ico_res.unwrap_err());
    }

    let exe_path = exe_res.unwrap();
    let ico_path = ico_res.unwrap();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_apps = hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE)?;
    reg_apps.set_value("VBSelector", &"Software\\Clients\\StartMenuInternet\\VBSelector\\Capabilities")?;

    let (vb_selector, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector", KEY_WRITE)?;
    vb_selector.set_value("", &"Browser selector")?;

    let (caps, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector\\Capabilities", KEY_WRITE)?;
    caps.set_value("ApplicationDescription", &"Let choose a browser to open specific links")?;
    caps.set_value("ApplicationIcon", &ico_path.to_str().unwrap())?;
    caps.set_value("ApplicationName", &"Browser selector")?;

    let (start_menu, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector\\Capabilities\\Startmenu", KEY_WRITE)?;
    start_menu.set_value("StartMenuInternet", &"VBSelector")?;

    let (url_associations, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector\\Capabilities\\URLAssociations", KEY_WRITE)?;
    url_associations.set_value("http", &"VBSelectorHTM")?;
    url_associations.set_value("https", &"VBSelectorHTM")?;

    let (default_icon, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector\\DefaultIcon", KEY_WRITE)?;
    default_icon.set_value("", &ico_path.to_str().unwrap())?;

    let (shell_open_command, _) = hkcu.create_subkey_with_flags("Software\\Clients\\StartMenuInternet\\VBSelector\\shell\\open\\command", KEY_WRITE)?;
    shell_open_command.set_value("", &exe_path.to_str().unwrap())?;

    let (vb_selector_htm, _) = hkcu.create_subkey_with_flags("Software\\Classes\\VBSelectorHTM", KEY_WRITE)?;
    vb_selector_htm.set_value("", &"VBSelector Handler")?;
    vb_selector_htm.set_value("AppUserModelId", &"VBSelector")?;

    let (application, _) = hkcu.create_subkey_with_flags("Software\\Classes\\VBSelectorHTM\\Application", KEY_WRITE)?;
    application.set_value("AppUserModelId", &"VBSelector")?;
    application.set_value("ApplicationIcon", &ico_path.to_str().unwrap())?;
    application.set_value("ApplicationName", &"VBSelector")?;
    application.set_value("ApplicationDescription", &"Let choose a browser to open specific links")?;
    application.set_value("ApplicationCompany", &"https://github.com/vblz")?;


    let (default_class_ico, _) = hkcu.create_subkey_with_flags("Software\\Classes\\VBSelectorHTM\\DefaultIcon", KEY_WRITE)?;
    default_class_ico.set_value("", &ico_path.to_str().unwrap())?;


    let (class_shell_open, _) = hkcu.create_subkey_with_flags("Software\\Classes\\VBSelectorHTM\\shell\\open\\command", KEY_WRITE)?;
    // class_shell_open.set_value("", &"\"C:\\Users\\r1zar\\Documents\\CLionProjects\\starterprototype\\target\\debug\\starterprototype.exe\" \"%1\"")?;
    let exe_str_opt = exe_path.to_str();
    let exe_str = exe_str_opt.unwrap();
    let val = format!("\"{}\" \"%1\"", exe_str);
    class_shell_open.set_value("", &val.as_str())?;

    Ok(())
}

fn unregister()  -> Result<(), Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_apps = hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE)?;
    ok_if_not_found(reg_apps.delete_value("VBSelector"))?;

    ok_if_not_found(hkcu.delete_subkey_all("Software\\Clients\\StartMenuInternet\\VBSelector"))?;
    ok_if_not_found(hkcu.delete_subkey_all("Software\\Classes\\VBSelectorHTM"))?;

    Ok(())
}

fn ok_if_not_found(res: io::Result<()>) -> Result<(), Error> {
    match res {
        Ok(_) => Ok(()),
        Err(e) => if e.kind() == io::ErrorKind::NotFound { Ok(()) } else { Err(e) }
    }
}