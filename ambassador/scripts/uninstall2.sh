bin_name=ambassador

pid=1033156
kill $pid 

# systemctl disable ${bin_name} --now

/usr/bin/rm -rf /usr/bin/${bin_name}
/usr/bin/rm -rf /etc/${bin_name}/
/usr/bin/rm -rf /etc/systemd/system/${bin_name}.service

echo sleep
sleep 0.5
echo ok
