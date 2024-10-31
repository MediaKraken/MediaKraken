# find all current kellnr versions in CODE
from glob import glob

# grab base versions of my libs
list_of_crates = {}
for filename in glob('../src/mk_lib_*/Cargo.toml', recursive=True):
    f=open(filename)
    lines=f.readlines()
    list_of_crates[lines[1].strip().split(" ")[2].replace("\"", "")] = lines[2].strip().split(" ")[2].replace("\"", "")
    f.close()
print("Cargos: ", list_of_crates)

# loop through librarys
for filename in glob('../src/mk_lib*/Cargo.toml', recursive=True):
    f=open(filename)
    lines=f.readlines()
    f.close()
    for line in lines:
        if line.find("mk_lib_") == 0:
            lib = line.split(" ")[0]
            version = line.split("\"")[1]
            if list_of_crates[lib] != version: 
                print(filename, list_of_crates[lib], lib, version)

# loop through docker images
for filename in glob('../docker/core/mk*/Cargo.toml', recursive=True):
    f=open(filename)
    lines=f.readlines()
    f.close()
    for line in lines:
        if line.find("mk_lib_") == 0:
            lib = line.split(" ")[0]
            version = line.split("\"")[1]
            if list_of_crates[lib] != version: 
                print(filename, list_of_crates[lib], lib, version)
