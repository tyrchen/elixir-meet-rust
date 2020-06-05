:dets.open_file(:dets_db, [{:file, '/tmp/db.txt'}])
sled_db = Rsled.open("/tmp/test_db")

Benchee.run(%{
  "dets_db.write" => fn -> :dets.insert(:dets_db, {:a, UUID.uuid1()}) end,
  "sled_db.write" => fn -> Rsled.put(sled_db, "a", UUID.uuid1()) end
})
