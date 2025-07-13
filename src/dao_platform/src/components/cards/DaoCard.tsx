import { formatDate } from "@/utils/date";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Dao } from "@/models/entities/Dao";
import { useNavigate } from "react-router";
import { DaoStatusBadge } from "../badge/DaoStatusBadge";
import { Role } from "@/models/entities/User";

interface DaoCardProps {
    dao: Dao;
}

export const DaoCard = ({ dao }: DaoCardProps) => {
    const navigate = useNavigate();

    return (
        <Card className="h-full flex flex-col">
            <CardHeader className="pb-2">
                <div className="flex justify-between items-start">
                    <div>
                        <CardTitle className="text-2xl font-bold">{dao.name}</CardTitle>
                        <CardDescription className="mt-1 h-16 overflow-hidden text-ellipsis line-clamp-3">{dao.purpose}</CardDescription>
                    </div>
                    <DaoStatusBadge status={dao.status} />
                </div>
            </CardHeader>
            <CardContent>
                <div className="space-y-4 text-sm">
                    <div className="flex flex-col">
                        <p className="text-slate-500 mb-1">Legal Form</p>
                        <p className="font-medium">{dao.legalForm}</p>
                    </div>

                    <div className="flex flex-col">
                        <p className="text-slate-500 mb-1">Location</p>
                        <p className="font-medium">{dao.town}, {dao.zip}</p>
                    </div>

                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <p className="text-slate-500 mb-1">Board</p>
                            <p className="font-medium">{dao.members.filter(member => member.role === Role.Board).length} members</p>
                        </div>
                        <div>
                            <p className="text-slate-500 mb-1">Members</p>
                            <p className="font-medium">{dao.members.filter(member => member.role === Role.Member).length}</p>
                        </div>
                    </div>

                    <div className="flex flex-col">
                        <p className="text-slate-500 mb-1">Created</p>
                        <p className="font-medium">{formatDate(dao.createdAt.getTime())}</p>
                    </div>

                    <div className="flex flex-col">
                        <p className="text-slate-500 mb-1">ID</p>
                        <p className="font-medium text-xs truncate">{dao.uid}</p>
                    </div>
                </div>
            </CardContent>
            <CardFooter>
                <Button variant="outline" className="w-full" onClick={() => navigate(`/daos/${dao.principal.toText()}`)}>View Details</Button>
            </CardFooter>
        </Card>
    );
}