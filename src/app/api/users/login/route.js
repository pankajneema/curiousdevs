import { connect } from "@/dbConfig/dbConfig";
import User from "@/models/userModel";
import { NextRequest ,NextResponse } from "next/server";
import bcryptjs from "bcryptjs";
import jwt from "jsonwebtoken";
import { tokenToString } from "typescript";
connect()

export async function POST(request){
  try{
    const reqBody = await request.json()
    console.log("Login :: reqBody",reqBody);
    const {username , password} = reqBody;

    //checl alredy Exits
    const user = await User.findOne({username});
    console.log("Login :: check User Result :",user)
    if (!user){
        return NextResponse.json({error: "User does  not exits Please SignUp" ,status:400 })
    }
    

    //check the password 
    const validPassword  = await bcryptjs.compare(password,user.password)
    //password hash
    if(!validPassword){
        return NextResponse.json({error: "Invalid Password" ,status:400 })
    }
    
    //create the token 
    const TokenData = {
        id: user._id,
        username: user.username,
        email: user.email
    }
    const token = await jwt.sign(TokenData,process.env.JWT_SECRET_KEY , {expiresIn: "2h"})
    
    const response  =  NextResponse.json({
        message:"LoggedIn Succesfully",
        status:true,
    })
    response.cookies.set("token",token,{
        httpOnly:true
    })
    return response;
  }
  catch(error){
    return NextResponse.json({error: error.message},{status:500})
  }
} 