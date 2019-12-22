(ns async.core
  (:require [clojure.core.async :as a])
  (:import (java.io BufferedReader
                    InputStreamReader)
           (java.net URL)
           (javax.net.ssl HttpsURLConnection))
  (:gen-class))

(defn fetch-url [url]
  (with-open [br (-> (URL. url)
                     .openConnection .getInputStream
                     InputStreamReader. BufferedReader.)]
    (loop [bytes 0]
      (let [line (.readLine br)]
        (if (.ready br) (recur (+ bytes (.length line)))
          (do (.close br)
              bytes))))))

(defn fetch-all-sync [url n]
  (reduce + (repeatedly n #(fetch-url url))))

(defn fetch-all-async [url n]
  (let [ch (a/merge (repeatedly n #(a/go (fetch-url url))))]
    (loop [size 0]
      (let [n (a/<!! ch)]
        (if n (recur (+ size n))
          size)))))

(defn time-this [label f]
  (let [start-time (System/nanoTime)
        result (f)
        delta-ms (-> (System/nanoTime) (- start-time) (/ 1000000))]
    (println (str label ": " result " in " (float delta-ms) "ms"))
    result))

(defn -main
  ([] (println "Please provide URL to access"))
  ([url & args]
   (let [req-count 500
         sync-chan (a/go (time-this "sync" #(fetch-all-sync url req-count)))
         async-chan (a/go (time-this "async" #(fetch-all-async url req-count)))]
     (a/<!! (a/into [] (a/merge [sync-chan async-chan]))))))

