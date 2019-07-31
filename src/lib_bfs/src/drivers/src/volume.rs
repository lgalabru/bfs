use crate::file::File

pub struct Volume {

}

impl Volume {

    fn create_file() {
    }

    fn read_file() {
    }

    fn update_file() {
    }

    fn delete_file() {
    }

    fn share_file() {
    }

    fn init_volume() {
    }

    fn get_config() {
    }
}

// /volume.config (public, signed, encrypted)
// {
//     name: "name",
// }
// /volume.status (public, signed, encrypted)
// {
//     name: "name",
// }
// /volume/index (public, signed, encrypted)
// {
//     files: {
//         "path/to/file.ext": {
//             encr: true,
//             sign: true,
//             size: 1023,
//             c_at: ,
//             u_at: ,
//         }
//     }
// }
// /volume/acl/STA102937843981029318293 (public, signed, encrypted)
// {
//     config: "<encrypted-credentials>":
//     name: ""
//     image_url: ""
//     can_read: {
//         files: [

//         ]
//     }
//     sk:
// }
// /volume/keychain (private, signed, encrypted)
// {
//     "public_key": "<encrypted-credentials>"
// }
// /inbox/STA102937843981029318293 (protected, signed, encrypted)
// /addressbook/STA102937843981029318293 (public, signed, not encrypted)
// /cache/addressbook/STA102937843981029318293 (public, signed, not encrypted)
// /cache/keys/STA102937843981029318293 (public, signed, encrypted)



// anyone can write in /inbox

// /volumes

