bin_name=ambassador

echo aa1 >> ~/aa.txt
systemctl disable ${bin_name}
echo aa2 >> ~/aa.txt

echo aa3 >> ~/aa.txt
/usr/bin/rm -rf /usr/bin/${bin_name}
echo aa4 >> ~/aa.txt
/usr/bin/rm -rf /etc/${bin_name}/
echo aa5 >> ~/aa.txt
/usr/bin/rm /etc/systemd/system/${bin_name}.service
echo aa6 >> ~/aa.txt

systemctl stop ${bin_name}
echo aa7 >> ~/aa.txt
systemctl daemon-reload
echo aa7 >> ~/aa.txt

# rm log ?
