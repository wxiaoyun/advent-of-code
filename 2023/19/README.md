# [ Day 19: Aplenty ](https://adventofcode.com/2023/day/19)

--- Day 19: Aplenty ---The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someonestolefrom Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.As you reach the bottom of therelentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have asystem.To start, each part is rated in each of four categories:x: Extremely cool lookingm:Musical (it makes a noise when you hit it)a:Aerodynamics:ShinyThen, each part is sent through a series ofworkflowsthat will ultimatelyacceptorrejectthe part. Each workflow has a name and contains a list ofrules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)Consider the workflowex{x>10:one,m<20:two,a>30:R,A}. This workflow is namedexand contains four rules. If workflowexwere considering a specific part, it would perform the following steps in order:Rule "x>10:one": If the part'sxis more than10, send the part to the workflow namedone.Rule "m<20:two": Otherwise, if the part'smis less than20, send the part to the workflow namedtwo.Rule "a>30:R": Otherwise, if the part'sais more than30, the part is immediatelyrejected(R).Rule "A": Otherwise, because no other rules matched the part, the part is immediatelyaccepted(A).If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part isaccepted(sent toA) orrejected(sent toR), the part immediately stops any further processing.The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow namedin. In this example, the five listed parts go through the following workflows:{x=787,m=2655,a=1222,s=2876}:in->qqz->qs->lnx->A{x=1679,m=44,a=2067,s=496}:in->px->rfg->gd->R{x=2036,m=264,a=79,s=2244}:in->qqz->hdj->pv->A{x=2461,m=1339,a=466,s=291}:in->px->qkq->crn->R{x=2127,m=1623,a=2188,s=1013}:in->px->rfg->AUltimately, three parts areaccepted. Adding up thex,m,a, andsrating for each of the accepted parts gives7540for the part withx=787,4623for the part withx=2036, and6951for the part withx=2127. Adding all of the ratings forallof the accepted parts gives the sum total of19114.Sort through all of the parts you've been given;what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?