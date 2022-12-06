- Todo
  - Batched search
  - Cache on memory
    - The engine returns names of the files, not its content. That means I have to open the same file over and over per each search. Doesn't that seem inefficient?
  - Pipeline
    - If the search result is big, it can be pipelined.
    - The result is returned in stream, through pipe.
      - Let's use multiprocessing module from my game engine!
  - Incremental update
    - file 하나 추가/삭제될 때마다 index 통째로 update하기 귀찮지?
    - 새로 추가된 파일만 반영 가능하게 고치기!
    - 검색하면서 존재하지 않는 파일들은 무시하기

- K-V DBs
  - My own
  - sled
    - 99%
  - TiKV
  - rust-rocksdb

---

search keywords

3 letters -> hash_3
4 letters -> hash_3[0:3], hash_3[1:4]
5 letters -> hash_5
6 letters -> hash_5[0:5], hash_5[1:6]
7 letters -> hash_5[0:5], hash_5[1:6], hash_5[2:7]

---

나중에 power of 2를 그대로 쓸 순 없으니까, (그럼 hash_5를 2^30^ row짜리 table에 넣어야함) 걔네를 적절히 mod 해줘야하지? 그 mod할 숫자들: floor(2^n^ / 10)로 하면 됨! 근데 웬만해선 홀수로 하자

[13107, 104857, 209715, 1677721, 3355443, 26843545, 53687091, 429496729, 858993459]

참고로 hash_3가 2^24^이고 hash_5가 2^30^인데 26843545가 2^24^보다 조금 더 크고 858993459가 2^30^보다 조금 더 작음