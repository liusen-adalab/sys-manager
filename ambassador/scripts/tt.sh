echo "pid = $1"

sleep 1
echo "killing"
kill $1

sleep 1
echo "killed $?"

exit 0
echo "killed $?"