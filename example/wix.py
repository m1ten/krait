# Wix general information
name = 'wix' # name of wix (not case sensitive)
author = 'miten' # author of wix
version = '0.1.0' # version of wix
description = 'wix - cross platform package manager' # description of wix (optional)
license = 'zlib' # license of wix 
repository = 'https://github.com/m1ten/wix' # repository of wix 

# Wix package information
mirrors = [''] # Mirror list in preferred order (optional)
installed = [''] # Installed packages 
dir_nix = '~/wix' # Directory of wix on nix
dir_win = 'C:\\wix' # Directory of wix on windows
install_dir_nix = f'{dir_nix}/bin' # Installation directory for nix
install_dir_win = f'{dir_win}\\bin' # Installation directory for windows
cache_dir_nix = f'{dir_nix}/cache' # Cache directory for nix
cache_dir_win = f'{dir_win}\\cache' # Cache directory for windows
temp_dir_nix = f'{dir_nix}/temp' # Temporary directory for nix
temp_dir_win = f'{dir_win}\\temp' # Temporary directory for windows