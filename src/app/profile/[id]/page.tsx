export default function UserProfile(params: any){
    return(
        <div>
            <h1>Profie Page</h1>
            <h1> Your Name : {params.id}
                {/* <span className="bg-brown text-Red rounded-md px-2 py-1"> */}
                   {params.id}
                {/* </span> */}
            </h1>
        </div>
    )
}