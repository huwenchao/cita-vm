env.debug('Testing: env.debug')
env.debug('Test[v]: env.debug')

env.debug('Testing: env.ret')
var buffer_ret = new Buffer([0x54, 0x65, 0x73, 0x74, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x2e, 0x72, 0x65, 0x74])
env.ret(buffer_ret)
env.debug('Test[v]: env.ret')
