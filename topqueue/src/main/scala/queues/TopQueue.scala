package topqueue

import collection.mutable.Builder
import collection.mutable.PriorityQueue


class TopQueue[A](val capacity: Int)(implicit val ord: Ordering[A])
    extends Builder[A, Seq[A]] {

  private[this] val queue = new PriorityQueue[A]()(ord.reverse)

  override def +=(elem: A): this.type = {
    if (queue.size < capacity) {
      queue.enqueue(elem)
    } else if (ord.gt(elem, queue.head)) {
      val _ = queue.dequeue
      queue.enqueue(elem)
    }
    this
  }

  def length: Int = queue.length
  def size: Int = length

  override def clear: Unit = queue.clear
  override def result: Seq[A] = queue.dequeueAll.reverse
}

object TopQueue {
  def apply[A](size: Int)(implicit ord: Ordering[A]): TopQueue[A] =
    new TopQueue[A](size)(ord)

  def apply[A](size: Int, coll: Iterable[A])(implicit ord: Ordering[A]): TopQueue[A] = {
    val tq = new TopQueue[A](size)(ord)
    coll.foreach(tq += _)
    tq
  }

  def apply[A](size: Int, count: Int)(block: => A)(implicit ord: Ordering[A]): TopQueue[A] = {
    val tq = new TopQueue[A](size)(ord)
    for (_ <- 1 to count) {
      tq += block
    }
    tq
  }
}

package object utils {
  implicit class IterableWithTop[A, CA <% Iterable[A]](val iter: CA) {
    def top(capacity: Int)(implicit ord: Ordering[A]): Seq[A] =
      TopQueue(capacity, iter)(ord).result
  }
}

