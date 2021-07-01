; Clojure example returning the first 100 odd integers
(defn n-odds [n]
  (->> (range)
       (filter odd?)
       (take n)))

(partition 10 (n-odds 100))
