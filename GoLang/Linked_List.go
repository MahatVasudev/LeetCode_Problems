package main

import "fmt"

type LinkedList[K comparable] struct {
	data    K
	pointer *LinkedList[K]
}

func (ll *LinkedList[K]) AppendNode(data K) {
	newLinkedList := &LinkedList[K]{
		data:    data,
		pointer: nil,
	}

	for ll.pointer != nil {
		ll = ll.pointer
	}

	ll.pointer = newLinkedList
}

func (ll *LinkedList[K]) Print() {
	for ll != nil {
		fmt.Printf("| %v| %v| -> ", ll.data, &ll.pointer)
		ll = ll.pointer
	}
}

func NewLinkedList[K comparable](data K) *LinkedList[K] {
	return &LinkedList[K]{
		data:    data,
		pointer: nil,
	}
}

func main() {
	ll := NewLinkedList[int32](1)
	ll.AppendNode(2)
	ll.AppendNode(3)
	ll.AppendNode(4)
	ll.AppendNode(5)
	ll.Print()
}
