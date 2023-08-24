bin_name=ambassador

systemctl disable ${bin_name} --now

/usr/bin/rm -rf /usr/bin/${bin_name}
/usr/bin/rm -rf /etc/${bin_name}/
/usr/bin/rm /etc/systemd/system/${bin_name}.service

# rm log ?