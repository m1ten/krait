--           Krait Config           
--      READ THE DOCUMENTATION   
-- You may use any syntax, but it is recommended to use the following syntax for consistency.
krait.config = {
    -- Most of these are optional and have default values.
    name = "krait",
    ver = "0.0.1",
    license = "Apache-2.0",
    git = "https://github.com/m1ten/krait",
    repos = {"https://github.com/m1ten/kraits"},
    args = {
        allow_all = true,
        -- Some arguments are internal use only.
        status = "install"
    }
}

-- You can use built-in functions to set variables
local os = krait.os.name()
if os == "windows" then
    krait.config.dir = "C:\\Users\\miten\\krait\\"
elseif os == "linux" then
    krait.config.dir = "/home/miten/krait/"
end

-- You can set package variables in the same way.
krait.pkg = {
    {
        name = "luau",
        dev = "Roblox",
        -- Alternatively, you can use "ver" instead of "version". (Aliases are supported)
        version = "0.0.1"
    }, -- You can also use tables to set multiple packages. (Not recommended)
    {name = "rust", dev = "Mozilla", license = {"MIT", "Apache-2.0"}}
}

function linux_install()
    -- You can write the manual installation code here or use the built-in functions.
    -- Some packages may require manual scripting.

    -- You can print and scan from the console.
    krait.io.print("Press enter to continue...")
    while krait.io.key_detect("ENTER") do
        -- Sleep will pause the script for a specified amount of time.
        krait.sleep(0.1)
    end

    -- The packages script can request the user for permissions.
    -- The require function will continue if the user accepts the permission.
    -- Otherwise, the script will exit.
    krait.perms.require("root")
end

-- Reuse the os variable from above.
if os == "linux" then
    krait.pkg[1].install = linux_install
else -- Using else is discouraged. Use elseif instead because not all operating systems are supported.
    -- You can also use the built-in functions to install packages.
    -- This might use the system package manager.
    krait.pkg[1].install = krait.install(krait.pkg[1])
end

-- Although defining non-standard variables under krait is not recommended, it is possible.
krait.kaboom = "BOOM!"

-- The standard library is disabled by default (for security reasons) but can be enabled under krait.config.
krait.config.stdlib = {
    allow = true, -- boolean or specify the functions you want to allow.
    force = true -- Force the standard library to be used.
}

-- Whenever possible, use the krait built-in functions.
-- The standard library is slower than the krait functions.
if krait.config.stdlib.force then
    io.write(krait.kaboom)
else
    krait.io.print(krait.kaboom)
end
