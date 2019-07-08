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

var addr = env.address()
if (addr[19] == 0x01) {
    env.debug('Test[v]: env.address')
} else {
    env.debug('Test[x]: env.address')
}
