
const router = require('express-async-router').AsyncRouter()
module.exports = router

const catalogHandler = require('./catalog.js')
const serviceInstancesHandler = require('./service_instances.js')

router.get('/catalog', catalogHandler)
router.get('/service_instances', serviceInstancesHandler)