sudo vgextend vgk8s1 /dev/sde \
&& sudo lvextend -l +100%FREE /dev/vgk8s1/vgk8s1lv \
&& sudo resize2fs /dev/vgk8s1/vgk8s1lv