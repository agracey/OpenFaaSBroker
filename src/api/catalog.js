const catalogHandler = async(req, res) => {

  return {
    services: [
      {
        name:"Example Service",
        id: "Example",
        description: "A Service to show how OSBA Works",
        tags: ["example"],
        requires: [],
        bindable: true,
        instances_retrievable: true,
        bindings_retrievable: true,
        allow_context_updates: true,
        metadata:{},
        dashboard_client:{
          id:'blah',
          secret:'faas',
          redirect_uri:'http://suse.com'
        },
        plan_updateable: true,
        plans: [
          {
            id: 'planid',
            name: 'plan name',
            description:'Sample Plan',
            free:true,
            bindable: true,
            schemas:{},
            maximum_polling_duration:30,
            maintenance_info:{
              version: '1',
              description:'mi desc'
            }
          }
        ]

      }
    ]
  }
}


module.exports = catalogHandler