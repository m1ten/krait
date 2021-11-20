# Start of file

# import the wix module (optional, if not imported, wix.X will error)
# using wix.X instead of python.X is optional, but recommended for speed
import wix 

# Package information (variable names are case sensitive, but not the values)
# file name must be the same as the package name
name = "Dash" # name of the package (not case sensitive)
alias = "dash-wix" # alias of the package (optional)
version = "0.1.0" # version of the package
description = "Dash is an OS setup utility." # description of the package (optional)
license = "zlib" # license of the package (optional)
maintainer = "user" # maintainer of the package (optional)
maintainer_email = "user@test.com" # maintainer email (optional)
website = None # website of the package (optional)
url = "https://github.com/m1ten/dash" # Download url of the package 
mirrors = ["pkg-mirror-1", "pkg-mirror-2"] # Mirror list in preferred order (optional)
dependency = "wix" # For one dependency, use this variable name
dependencies = ["pkg-dep-1", "pkg-dep-1"] # Multiple dependency list 
optional_depedency = None # For one optional dependency, use this variable name
optional_dependencies = ["pkg-opt-dep-1", "pkg-opt-dep-2"] # Multiple optional dependencies
supported_os = ["macOS", "Linux", "Windows"] # Supported OS list
# For a full list of supported variables, see https://m1ten.github.io/wix/

def install(): # Manual installation 
	wix.print("Installing Dependencies")
	wix.print("Installing Dash")

def uninstall(): # Manual uninstallation
	wix.print("Uninstalling Dash")
	wix.print("Uninstalling Dependencies")

def random_function(): # Random functions are allowed (not recommended)
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