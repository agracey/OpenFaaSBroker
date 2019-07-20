
const storeUrl = process.env.STORE_URL || 'https://raw.githubusercontent.com/openfaas/store/master/functions.json';
const fetch = require("node-fetch");


const buildSchema = (f) => {


  const envOptionsSchema = Object.keys(f.environment||{})
    .reduce((acc, curr)=>({...acc,[curr]:{type:"string"}}),{})

  const schema = {
    type:"object",
    properties: {
      environment:{type:"object", properties:envOptionsSchema}
    }
  }

  return {
    service_instance:{
      create:schema,
      update:schema
    },
    service_binding:{
      create:schema
    }
  }
}

const buildPlansForArch = (f) => {
  return Object.keys(f.images).map((arch)=>({
    id: arch,
    name: arch,
    description:'Run Function in Arch: ' + arch,
    free:true,
    bindable: true,
    schemas:buildSchema(f),
    maximum_polling_duration:30,
    maintenance_info:{
      version: '1',
      description:'mi desc'
    }
  }))
}

const transformFunction = (f) => {
  return {
    name: f.title,
    id: f.name, //TODO: GUID?
    description:f.description,
    tags: [ "Function" , ...Object.keys(f.images)],
    bindable: true,
    instances_retrievable: true,
    bindings_retrievable: true,
    allow_context_updates: false,
    plan_updateable: true,
    plans: buildPlansForArch(f)

  }
}

const catalogHandler = async(req, res) => {

const data = await fetch(storeUrl).then(res=>(res.json()))

return {services:data.functions.map(transformFunction)}

}


module.exports = catalogHandler