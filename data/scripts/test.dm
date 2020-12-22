export fn breaks() {
    export x

    panic("OH NOES D:")
}

export fn test() {
    breaks()
}
