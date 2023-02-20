# vintagestory_modpack_installer

The very beginnings of a Vintage Story modpack installer.

## Modpacks currently follow a structure.

The modpack folder can have any name but it requires a subfolder with a certain name.

This folder is required:
    
    /mods
  
These are optional:

    /optional_mods
    /README
    
The optional_mods folder allows for mods to be ignored by the user as they aren't needed for the modpack to function. This is often used for shader packs or other visual changes.

The README folder is completely ignored by the installer and allows the modpack to contain information available for users.


Example of a correctly formatted modpack:

    modpack_folder/
      mods/
        mod.zip
      optional_mods/
        preference_based_mod.zip
      README/
        information.txt
