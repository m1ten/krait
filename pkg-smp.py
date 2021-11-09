# Start of file

import wix # wix module

# Package information
name = "Dash"
version = "0.1.0"
description = "Dash is an OS setup utility."
license = "zlib"
maintainer = "user"
maintainer_email = "user@test.com"
website = None
url = "https://github.com/m1ten/dash" # Download url
mirrors = ["pkg-mirror-1", "pkg-mirror-2"] # Mirror list in preferred order
dependency = "wix" # For one dependency, use this
dependencies = ["pkg-dep-1", "pkg-dep-1"] # Dependency list
optional_depedency = None # For one optional dependency, use this
optional_dependencies = ["pkg-opt-dep-1", "pkg-opt-dep-2"] # Optional dependencies
supported_os = ["macOS", "Linux", "Windows"] # Supported OS list
# For a full list of supported variables, see https://m1ten.github.io/wix/

def install(): # Manual installation 
	wix.print("Installing Dependencies")
	wix.print("Installing Dash")

def uninstall(): # Manual uninstallation
	wix.print("Uninstalling Dash")
	wix.print("Uninstalling Dependencies")

def random_function(): # Random functions are allowed
	global name # Access global variables
	random_variable = "random" # Random variables are allowed
	name = "Not Dash" # If this function is called, new value will replace the old one 
	wix.print("Random function")

# Note: this function will be called everytime this file is accessed by wix 
# Avoid calling functions, it will slow down the process
random_function()

name = "Dash++" # This will replace the value of name variable again 

# Do not call the install/uninstall functions, they will be called by wix

# End of file