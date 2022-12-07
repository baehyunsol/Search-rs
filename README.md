- Todo
  - Batched search
  - Cache on memory
    - The engine returns names of the files, not its content. That means I have to open the same file over and over per each search. Doesn't that seem inefficient?
  - Pipeline
    - If the search result is big, it can be pipelined.
    - The result is returned in stream, through pipe.
      - Let's use multiprocessing module from my game engine!
  - Incremental update
    - append/remove file incrementally

---

나중에 power of 2를 그대로 쓸 순 없으니까, (그럼 hash_5를 2^30^ row짜리 table에 넣어야함) 걔네를 적절히 mod 해줘야하지? 그 mod할 숫자들: floor(2^n^ / 10)로 하면 됨! 근데 웬만해선 홀수로 하자

[13107, 104857, 209715, 1677721, 3355443, 26843545, 53687091, 429496729, 858993459]

참고로 hash_3가 2^24^이고 hash_5가 2^30^인데 26843545가 2^24^보다 조금 더 크고 858993459가 2^30^보다 조금 더 작음