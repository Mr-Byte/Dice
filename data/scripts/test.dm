export fn breaks() {
    let x = 30
    export x

    panic("OH NOES D:")
}

export fn test() {
    breaks()
}
