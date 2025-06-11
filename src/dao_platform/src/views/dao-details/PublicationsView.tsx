import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { Calendar, File, FileText } from "lucide-react";
import { SogcPublication } from "@/models/entities/SogcPublication";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { formatDate } from "@/utils/date";
import { ExpandibleText } from "@/components/texts/ExpandibleText";
import { Badge } from "@/components/ui/badge";

interface PublicationsViewProps {
    sogcPublications: SogcPublication[];
}

export const PublicationsView = ({ sogcPublications }: PublicationsViewProps) => {
    return (
        <Card>
            <CardHeader>
                <CardTitle>SOGC Publications</CardTitle>
                <CardDescription>Official publications in the Swiss Official Gazette of Commerce</CardDescription>
            </CardHeader>
            <CardContent>
                {sogcPublications && sogcPublications.length > 0 ? (
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>Daily Number</TableHead>
                                <TableHead>Publication Date</TableHead>
                                <TableHead>Mutation</TableHead>
                                <TableHead>Description</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {sogcPublications.map((publication) => (
                                <TableRow key={publication.sogcId}>
                                    <TableCell>{publication.dailyNumber}</TableCell>
                                    <TableCell>
                                        <div className="flex items-center gap-2">
                                            <Calendar className="h-4 w-4 text-slate-500" />
                                            <div>
                                                <p className="font-medium">{formatDate(publication.publicationSogcDate.getTime())}</p>
                                                <p className="text-xs text-slate-500">
                                                    Created: {formatDate(publication.publicationDate.getTime())}
                                                </p>
                                            </div>
                                        </div>
                                    </TableCell>
                                    <TableCell>
                                        <div className="flex items-center gap-2">
                                            <FileText className="h-4 w-4 text-slate-500 flex-shrink-0" />
                                            <div className="min-w-0 flex flex-wrap items-center">
                                                {publication.mutations.map((mutation, idx) =>
                                                    <Badge key={idx} variant="secondary" className="mr-1 text-xs">
                                                        {mutation}
                                                    </Badge>)}
                                            </div>
                                        </div>
                                    </TableCell>
                                    <TableCell className="max-w-md">
                                        <ExpandibleText text={publication.description} />
                                    </TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                ) : (
                    <div className="text-center py-8 text-slate-500">
                        <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                        <h3 className="font-medium mb-1">No SOGC Publications</h3>
                        <p>There are no publications in the Swiss Official Gazette of Commerce for this DAO.</p>
                    </div>
                )}
            </CardContent>
        </Card>
    )
};
