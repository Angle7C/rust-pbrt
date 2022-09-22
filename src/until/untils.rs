pub fn quadratic(a:f32,b:f32,c:f32)->Option<(f32,f32)>{
    let det=b*b-4.00*a*c;
    if det<0.0{
        None
    }else{
        let mut t0=(-b-det.sqrt())/(2.0*a);
        let mut t1=(-b+det.sqrt())/(2.0*a);
        if t0>t1{
            std::mem::swap( &mut  t0,  &mut t1);
        };
        Some((t0,t1))
    }
}