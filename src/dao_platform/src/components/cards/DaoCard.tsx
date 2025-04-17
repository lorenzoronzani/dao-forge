import { DAO } from "@/pages/Dashboard";
import { Button } from "../ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "../ui/card";

interface DaoCardProps {
    dao: DAO;
}

export const DaoCard = ({ dao }: DaoCardProps) => {
    return (
        <Card>
            <CardHeader>
                <CardTitle>{dao.name}</CardTitle>
                <CardDescription>{dao.description}</CardDescription>
            </CardHeader>
            <CardContent>
                <div className="flex justify-between text-sm">
                    <div>
                        <p className="text-slate-500">Members</p>
                        <p className="font-medium">{dao.members}</p>
                    </div>
                    <div>
                        <p className="text-slate-500">Assets</p>
                        <p className="font-medium">{dao.assets}</p>
                    </div>
                </div>
            </CardContent>
            <CardFooter>
                <Button variant="outline" className="w-full">View Details</Button>
            </CardFooter>
        </Card>
    );
}