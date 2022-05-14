## Crates tester
This app is simple tool to help develop [Czkawka](https://github.com/qarmin/czkawka) app, by helping develop that Czkawka use like Infer, Image-rs, Lofty.

I strongly advice to run this tool to check for files, to be able to report bug inside proper repository to 

Available modes:
- `image` - opens image and check if is proper  
https://github.com/image-rs/image
- `lofty` - opens audio things, finds corrupted files or without length/bitrate property  
  https://github.com/Serial-ATA/lofty-rs
- `infer` - finds files which type cannot be guessed by type  
  https://github.com/bojand/infer
- `mime` - shows files which extensions doesn't have assigned mime type
  https://github.com/abonander/mime_guess
- `zip` - finds invalid zip files
  https://github.com/zip-rs/zip

Before using, be sure that you are using the latest version of libraries(`cargo update` will do this for you)

Putting at the end of arguments - `PRINT` will print every single broken file. It is available in `mime`, `infer` modes, where it is possible that results could several thousands.

### Example usage
`cargo run --release image /home /mnt/Miecz` - will scan `/home` and `/mnt/Miecz` directories for invalid/unsupported images
`cargo run --release mime /home/raczek PRINT` - 
