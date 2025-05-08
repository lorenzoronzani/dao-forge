import { Principal } from "@dfinity/principal";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Users } from "lucide-react";
import { Badge } from "../ui/badge";

interface MemberCardProps {
    title: string;
    description: string;
    members: Principal[];
}

export const MemberCard = ({ title, description, members }: MemberCardProps) => {
    return (
        <Card>
            <CardHeader>
                <div className="flex items-center justify-between">
                    <CardTitle className="mb-1">{title}</CardTitle>
                    <Badge variant="outline" className="font-medium">
                        {members.length} {members.length === 1 ? 'member' : 'members'}
                    </Badge>
                </div>
                <CardDescription>{description}</CardDescription>
            </CardHeader>
            <CardContent>
                {members.length > 0 ? (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Principal ID</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {
                                members.map((member, index) => (
                                    <TableRow key={index}>
                                        <TableCell className="font-mono text-xs">{member.toText()}</TableCell>
                                    </TableRow>
                                ))
                            }
                        </TableBody>
                    </Table>
                ) : (
                    <div className="text-center py-8 text-slate-500">
                        <Users className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                        <h3 className="font-medium mb-1">No members</h3>
                        <p>This DAO doesn't have any members yet.</p>
                    </div>
                )}
            </CardContent>
        </Card>
    )
};