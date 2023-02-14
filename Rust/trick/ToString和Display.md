### Display

实现了Display后的类型就可以使用默认 `default formatter` 来打印输出，即使用 `"{}"`



### ToString

有时候我们并不需要打印输出某个类型，而需要将它转换为 `String` 类型，使用该 trait 即可。

当你实现了 `Display` 时你自然可以获得一个同样实现的 `ToString` ，因此你无需手动实现 `ToString`，当然你也可以手动实现 `ToString`，前提是不能实现 `Display`，因为如果你两个都实现的话编译器会告知冲突（即使你的实现是一样的）。

最佳实践：只实现 `Display`，然后不仅获得了打印的能力，还能白嫖一个 `ToString`