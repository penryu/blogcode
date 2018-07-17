package topqueue

import collection.mutable.Queue
import io.Source
import math.BigInt
import util.Random
import org.scalatest._


class TestTopQueue extends FlatSpec with Matchers {
  /** Scalatest class to test TopQueue data type */

  val rng = Random
  val InputSize = 10000
  val QueueSize = 100

  val lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."

  "TopQueue" should "handle being empty" in {
    val emptyQueue = TopQueue[Int](5)
    emptyQueue.size should equal (0)
    emptyQueue.result.size should equal (0)
  }

  it should "handle less than capacity" in {
    val input = Set(1, 5, 3)
    val shortQueue = TopQueue[Int](5)
    shortQueue ++= input
    shortQueue.size should equal (input.size)
    shortQueue.result.toSet should equal (input)
  }

  it should "handle capacity" in {
    val input = Set(1, 5, 3, 4, 2)
    val queue = TopQueue[Int](5)
    queue ++= input
    queue.size should equal (input.size)
    queue.result.toSet should equal (input)
  }

  it should "more than capacity values" in {
    val input = Set(1, 5, 3, 4, 2, 7, 6)
    val output = Set(7, 6, 5, 4, 3)
    val queue = TopQueue[Int](5)
    queue ++= input
    queue.size should equal (queue.capacity)
    queue.result.toSet should equal (output)
  }

  it should "invert sort appropriately" in {
    val nums = Seq.fill(20)(rng.nextInt)
    val sortedNums = nums.sorted

    val tq = nums.foldLeft(TopQueue[Int](3))(_ += _)
    val topNums = tq.result
    topNums.sorted should be (sortedNums.takeRight(3))

    implicit val inverseSort = Ordering[Int].reverse
    val mq = nums.foldLeft(TopQueue[Int](3))(_ += _)
    val minNums = mq.result
    minNums.sorted should be (sortedNums.take(3).reverse)
  }

  it should "correctly process Int types" in {
    val randomInts = Vector.fill(InputSize)(rng.nextInt(Int.MaxValue))
    val intTop = TopQueue(QueueSize, randomInts).result
    val (high, low) = randomInts.partition(_ >= intTop.min)

    intTop.size should equal (QueueSize)
    high.size should equal (QueueSize)
    low.size should equal (InputSize - QueueSize)
    intTop.toSet should equal (high.toSet)
    low.find(_ >= intTop.min) should be (None)
  }

  it should "correctly process BigInt types" in {
    val randomBigInts = Vector.fill(InputSize) {
      val r1 = rng.nextInt(Int.MaxValue)
      val r2 = rng.nextInt(Int.MaxValue)
      BigInt(r1) * r2
    }

    val bigIntTop = TopQueue(QueueSize, randomBigInts).result

    bigIntTop.size should equal (QueueSize)

    val (high, low) = randomBigInts.partition(_ >= bigIntTop.min)
    high.size should equal (QueueSize)
    low.size should equal (InputSize - QueueSize)
    bigIntTop.toSet should equal (high.toSet)
    low.find(_ >= bigIntTop.min) should be (None)
  }

  it should "correctly process generated types" in {
    val intTop = TopQueue(QueueSize, InputSize)(rng.nextInt(Int.MaxValue)).result
    intTop.size should equal (QueueSize)
  }

  it should "correctly process generated big integer types" in {
    val bigIntTop = TopQueue(QueueSize, InputSize)(BigInt(128, rng)).result
    bigIntTop.size should equal (QueueSize)
  }

  it should "empty the queue" in {
    val q = TopQueue(3, 1 to 10)
    q.length should equal (3)
    q.clear
    q.length should equal (0)
  }

  it should "correctly process String types" in {
    val QueueSize = 20

    val words = lorem.split(" ")

    /** sorts words in ASCII order */
    val wordsSortedASCII = words.sorted

    /** The following Ordering implements alphabetical order, overriding
      * the default ASCII sort order for Strings.
      * 
      * But multiple occurrences of the same word with different
      * capitalization will be sorted non-deterministically, so append
      * the original word to keep uppercase before lowercase.
      */
    implicit val ord = Ordering.by[String, (String, String)] {
      (word) => (word.toLowerCase, word)
    }.reverse
    /** ... so this sorts words alphabetically */
    val wordsSortedAlpha = words.sorted

    /** same, but uses implicit method top */
    import topqueue.utils._
    val topWords = words.top(QueueSize)

    /** TopQueue should get the proper number of values */
    topWords.size should equal (QueueSize)

    /** topWords would be the LAST [QueueSize] words */
    val lastWords = wordsSortedAlpha.takeRight(QueueSize)

    /** TopQueue should get the CORRECT values. */
    topWords.toSet should equal (lastWords.toSet)
  }

  "IterableWithTop" should "extend the Iterable type with top method" in {
    import topqueue.utils._
    val input = Set(51, 2, 80, 70, 35, 52, 14, 12, 43, 92)
    val output = Set(92, 80, 70)
    val top3 = input.top(3)
    top3.toSet should equal (output)
  }

}
