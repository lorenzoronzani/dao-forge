import { formatDate } from "@/utils/date";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Dao, OrganizationStatus } from "@/models/entities/Dao";

interface DaoCardProps {
    dao: Dao;
}

export const DaoCard = ({ dao }: DaoCardProps) => {
    const getStatusColor = (status: OrganizationStatus) => {
        switch (status) {
            case OrganizationStatus.Active:
                return "text-green-600 bg-green-50";
            case OrganizationStatus.Liquidation:
                return "text-yellow-600 bg-yellow-50";
            case OrganizationStatus.Dissolved:
                return "text-red-600 bg-red-50";
            default:
                return "text-gray-600 bg-gray-50";
        }
    };

    return (
        <Card className="h-full flex flex-col">
            <CardHeader className="pb-2">
                <div className="flex justify-between items-start">
                    <div>
                        <CardTitle className="text-2xl font-bold">{dao.name}</CardTitle>
                        <CardDescription className="mt-1 h-12 overflow-hidden">{dao.purpose}</CardDescription>
                    </div>
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(dao.status)}`}>
                        {dao.status}
                    </span>
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
                            <p className="font-medium">{dao.board.length} members</p>
                        </div>
                        <div>
                            <p className="text-slate-500 mb-1">Members</p>
                            <p className="font-medium">{dao.members.length}</p>
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
                <Button variant="outline" className="w-full">View Details</Button>
            </CardFooter>
        </Card>
    );
}