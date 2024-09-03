import { connect } from "@/dbConfig/dbConfig";
import User from "@/models/userModel";
import { NextRequest ,NextResponse } from "next/server";
import bcryptjs from "bcryptjs"

connect()

export async function POST(request){
  try{
    const reqBody = await request.json()
    console.log("reqBody",reqBody);
    const {username, email , password} = reqBody;

    //checl alredy Exits
    const user = await User.findOne({username});
    console.log("OLd Result :",user)
    if (user){
        return NextResponse.json({error: "User Already exits" ,status:400 })
    }

    //password hash
    const salt = await bcryptjs.genSalt(10);
    const hashPassword = await bcryptjs.hash(password,salt);
     
    //save new user 
    const newUser = await new User({
        username,
        email,
        password:hashPassword
    }).save();
    console.log("New User Result :",newUser)
    return NextResponse.json({
        message:"User Creaeted Succesfully",
        status:true,
        newUser
    })
  }
  catch(error){
    return NextResponse.json({error: error.message},{status:500})
  }
} 