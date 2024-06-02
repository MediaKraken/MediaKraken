cargo publish --registry kellnr -p mk_lib_hardware --allow-dirty --token=3dPvCZSd1V8xKpiCGqpXCBbkj4Jtyyqc

# web ui
http://mkkellrn:8000/

Order to Publish:  * has no deps
mk_lib_common *
mk_lib_compression *
mk_lib_filler *
mk_lib_image *
mk_lib_logging *
mk_lib_rabbitmq *

mk_lib_database
    mk_lib_common
mk_lib_file
    mk_lib_database
mk_lib_hash
    mk_lib_file
mk_lib_network
    mk_lib_file
mk_lib_hardware
    mk_lib_network
mk_lib_metadata
    mk_lib_common
    mk_lib_database
    mk_lib_hash
    mk_lib_network