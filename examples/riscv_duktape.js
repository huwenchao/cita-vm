env.debug('Testing: env.debug')
env.debug('Test[v]: env.debug')

env.debug('Testing: env.ret')
var buffer_ret = new Buffer([0x54, 0x65, 0x73, 0x74, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x2e, 0x72, 0x65, 0x74])
env.ret(buffer_ret)
env.debug('Test[v]: env.ret')

env.debug('Testing: env.save/env.load')
var buffer_k = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x6b])
var buffer_v = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x76])
env.save(buffer_k, buffer_v)

var r = env.load(buffer_k)
if (r.subarray(0, 5)[4] == buffer_v[4]) {
    env.debug('Test[v]: env.save/env.load')
} else {
    env.debug('Test[x]: env.save/env.load')
}

env.debug('Testing: env.address')
var addr = env.address()
if (addr[19] == 0x01) {
    env.debug('Test[v]: env.address')
} else {
    env.debug('Test[x]: env.address')
}

env.debug('Testing: env.balance')
var acc1 = new Buffer([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
var v = env.balance(acc1)
if (v[31] == 10) {
    env.debug('Test[v]: env.balance')
} else {
    env.debug('Test[x]: env.balance')
}

env.debug('Testing: env.origin')
var addr = env.origin()
if (addr[19] == 0x02) {
    env.debug('Test[v]: env.origin')
} else {
    env.debug('Test[x]: env.origin')
}

env.debug('Testing: env.caller')
var addr = env.caller()
if (addr[19] == 0x03) {
    env.debug('Test[v]: env.caller')
} else {
    env.debug('Test[x]: env.caller')
}

env.debug('Testing: env.callvalue')
var callvalue = env.callvalue()
if (callvalue[31] == 0x05) {
    env.debug('Test[v]: env.callvalue')
} else {
    env.debug('Test[x]: env.callvalue')
}

env.debug('Testing: env.blockhash')
var blockhash = env.blockhash(7)
if (blockhash[31] == 0x07) {
    env.debug('Test[v]: env.blockhash')
} else {
    env.debug('Test[x]: env.blockhash')
}

env.debug('Testing: env.coinbase')
var addr = env.coinbase()
if (addr[19] == 0x08) {
    env.debug('Test[v]: env.coinbase')
} else {
    env.debug('Test[x]: env.coinbase')
}

env.debug('Testing: env.timestamp')
var timestamp = env.timestamp()
if (timestamp == 0x09) {
    env.debug('Test[v]: env.timestamp')
} else {
    env.debug('Test[x]: env.timestamp')
}

env.debug('Testing: env.number')
var number = env.number()
if (number[31] == 0x06) {
    env.debug('Test[v]: env.number')
} else {
    env.debug('Test[x]: env.number')
}
