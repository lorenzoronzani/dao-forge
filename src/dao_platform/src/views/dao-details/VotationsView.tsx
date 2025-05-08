import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { File } from 'lucide-react';


interface VotationsViewProps {
    votations: any[];
}

export const VotationsView = ({ votations }: VotationsViewProps) => {
    return (
        <Card>
            <CardHeader>
                <div className="flex items-center justify-between">
                    <CardTitle>Votations</CardTitle>
                    <Button>Create votation</Button>
                </div>
                <CardDescription>Review and manage all voting processes</CardDescription>
            </CardHeader>
            <CardContent>
                {votations.length > 0 ? (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Title</TableHead>
                                <TableHead>Status</TableHead>
                                <TableHead>Start Date</TableHead>
                                <TableHead>End Date</TableHead>
                                <TableHead>Participation</TableHead>
                                <TableHead>Actions</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {votations.map((votation) => (
                                <TableRow key={votation.id}>
                                    <TableCell className="font-medium">{votation.title}</TableCell>
                                    <TableCell>
                                        <Badge
                                            className={
                                                votation.status === "Active" ? "bg-blue-100 text-blue-800" :
                                                    votation.status === "Completed" ? "bg-green-100 text-green-800" :
                                                        "bg-gray-100 text-gray-800"
                                            }
                                        >
                                            {votation.status}
                                        </Badge>
                                    </TableCell>
                                    <TableCell>{votation.startDate}</TableCell>
                                    <TableCell>{votation.endDate}</TableCell>
                                    <TableCell>{votation.participation}</TableCell>
                                    <TableCell>
                                        <Button variant="ghost" size="sm">View</Button>
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                ) : (
                    <div className="text-center py-8 text-slate-500">
                        <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                        <h3 className="font-medium mb-1">No votations yet</h3>
                        <p>Create a new votation to get started.</p>
                        <Button className="mt-4">Create votation</Button>
                    </div>
                )}
            </CardContent>
        </Card>
    )
}