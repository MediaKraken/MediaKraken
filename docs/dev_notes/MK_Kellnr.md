cargo publish --registry kellnr -p mk_lib_common --allow-dirty --token=5QcDi0JIg8gy2yfDy4aB44Pf3SouIbJH

# will ask for the token (generated in kellnr ui) - from src directory so .cargo exists
cargo login --registry kellnr

# web ui
http://mkdevkellnrapi.mediakraken.media:8000/

Order to Publish:  * has no deps
mk_lib_common *
mk_lib_compression *
mk_lib_filler *
mk_lib_image *
mk_lib_logging *
mk_lib_rabbitmq *
ed2k-rs *
weectrl *

mk_lib_database
    mk_lib_common
mk_lib_file
    mk_lib_database
mk_lib_hash
    mk_lib_file
    ed2k-rs
mk_lib_network
    mk_lib_file
mk_lib_hardware
    mk_lib_network
    ssdp-rs
    weectrl
mk_lib_metadata
    mk_lib_common
    mk_lib_database
    mk_lib_hash
    mk_lib_network