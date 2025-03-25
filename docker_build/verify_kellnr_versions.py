from glob import glob
import subprocess

def sed_file(filename):
    f=open(filename)
    lines=f.readlines()
    f.close()
    for line in lines:
        if line.find("mk_lib_") == 0:
            lib = line.split(" ")[0]
            old_version = line.split("\"")[1]
            print(filename, lib, old_version, list_of_crates[lib])
            if list_of_crates[lib] != old_version: 
                command_to_run = "sed -i 's/" + lib + " = { version = \"" + old_version + "\"/" + lib + " = { version = \"" + list_of_crates[lib] + "\"" + "/g' " + filename
                print(command_to_run)
                pid = subprocess.Popen(command_to_run, shell=True)
                pid.wait()

# find all current kellnr versions and bump level
list_of_crates = {}
for filename in glob('../src/mk_lib_*/Cargo.toml', recursive=True):
    f=open(filename)
    lines=f.readlines()
    old_version = lines[2].strip().split(" ")[2].replace("\"", "")
    crate_version = lines[2].strip().split(" ")[2].replace("\"", "").rsplit(".", 1)
    new_version = crate_version[0] + "." + str(int(crate_version[1]) + 1)
    list_of_crates[lines[1].strip().split(" ")[2].replace("\"", "")] = new_version
    f.close()
    command_to_run = "sed -i '3 s/version = \"" + old_version + "\"/version = \"" + new_version + "\"/g' " + filename
    pid = subprocess.Popen(command_to_run, shell=True)
    pid.wait()
print("My Kellnr Cargos: ", list_of_crates)

# loop through librarys
for filename in glob('../src/mk_lib*/Cargo.toml', recursive=True):
    sed_file(filename)

# loop through docker images
for filename in glob('../docker/core/mk*/Cargo.toml', recursive=True):
    sed_file(filename)
