## Crates tester
This app is simple tool to help develop [Czkawka](https://github.com/qarmin/czkawka) app, by finding bugs in libraries that Czkawka use like Infer, Image-rs, Lofty.

I recommend to check this tool on your own computer to increase number of checked types of files and later try to create issues in repositories bellow.

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
- `pdf` - finds invalid pdf files  
  https://github.com/pdf-rs/pdf

Before using, be sure that you are using the latest version of libraries(`cargo update` will do this for you)

Putting at the end of arguments - `PRINT` will print every single broken file. It is available in `mime`, `infer` modes, where it is possible that results could several thousands.

### Example usage
`cargo run --release image /home /mnt/Miecz` - will scan `/home` and `/mnt/Miecz` directories for invalid/unsupported images  
`cargo run --release mime /home/raczek PRINT` - additionally will print results about each file to output

### Example of reported bugs
https://github.com/abonander/mime_guess/issues/73  
https://github.com/bojand/infer/issues/61  
https://github.com/Serial-ATA/lofty-rs/issues/49  
https://github.com/image-rs/image/issues/1551  
https://github.com/pdf-rs/pdf/issues/133  
