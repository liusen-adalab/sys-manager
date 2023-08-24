bin_name=ambassador

pack_path=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

/usr/bin/cp ${pack_path}/${bin_name} /usr/bin/${bin_name}

mkdir -p /etc/${bin_name}/
/usr/bin/cp ${pack_path}/config.toml /etc/${bin_name}/

/usr/bin/cp ${pack_path}/${bin_name}.service /etc/systemd/system/
systemctl enable ${bin_name} --now

