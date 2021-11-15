package main
import (
	"fmt"
	"os"
	"strconv"
)

func main(){
	var x string = os.Args[1]
	n, error := strconv.Atoi(x)
	if error == nil {
		for i := 1; i < n + 1; i++ {
			if (i % 3 == 0) && (i % 5 == 0) {
				fmt.Println("FizzBuzz");
			} else if i % 3 == 0 {
				fmt.Println("Fizz");
			} else if (i % 5 == 0) {
				fmt.Println("Buzz");
			} else {
				fmt.Println(i);
			}
		}
	}
}