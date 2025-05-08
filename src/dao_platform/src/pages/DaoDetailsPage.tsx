import React, { useEffect, useState } from 'react';
import { ArrowLeft, Users, File, Calendar, Building, MapPin, FileText, ExternalLink } from 'lucide-react';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import TopBar from '@/components/headers/TopBar';
import { MainContainer } from '@/layouts/MainContainer';
import { BackButton } from '@/components/buttons/BackButton';
import { useNavigate, useParams } from 'react-router';
import { useDao } from '@/providers/DaoProvider';
import { Principal } from '@dfinity/principal';
import { OrganizationStatus } from '@/models/entities/Dao';
import { formatDate } from '@/utils/date';
import { paths } from '@/constants/paths';

export const DaoDetailsPage = () => {
    const navigate = useNavigate();
    const { id } = useParams();
    const { getDao } = useDao();

    const dao = getDao(Principal.fromText(id!));

    if (!dao) {
        navigate(paths.HOME);
    }

    const [activeTab, setActiveTab] = useState('members');
    const [showFullPurpose, setShowFullPurpose] = useState(false);

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

    // Dummy data for future features
    const votations: any[] = [
        { id: 1, title: "Board Member Election", status: "Completed", startDate: "2024-12-01", endDate: "2024-12-15", participation: "78%" },
        { id: 2, title: "Annual Budget Approval", status: "Active", startDate: "2025-01-10", endDate: "2025-01-25", participation: "42%" },
        { id: 3, title: "Strategic Partnership Decision", status: "Upcoming", startDate: "2025-02-05", endDate: "2025-02-20", participation: "-" },
    ];

    const publications: any[] = [
        // {
        //     id: 1,
        //     publicationDate: new Date(),
        //     title: "Publication 1",
        //     type: "Statute Change",
        //     status: "Published"
        // }
    ];

    return (
        <>
            <TopBar />

            <MainContainer>
                <BackButton onBack={() => navigate(-1)} />

                {/* DAO Header - Single row with essential info */}
                <div className="mb-6">
                    <div className="flex justify-between items-start mb-2">
                        <h1 className="text-2xl font-bold">{dao.name}</h1>
                        <Badge className={`${getStatusColor(dao.status)}`}>
                            {dao.status}
                        </Badge>
                    </div>

                    {/* Essential Information - Single row */}
                    <div className="bg-white p-4 rounded-lg shadow-sm mb-3">
                        <div className="grid grid-cols-2 md:grid-cols-4 gap-x-6 gap-y-2">
                            <div className="flex items-center gap-1">
                                <Building className="h-4 w-4 text-slate-500" />
                                <span className="text-slate-500 text-sm">Legal Form:</span>
                                <span className="text-sm ml-1">{dao.legalForm}</span>
                            </div>

                            <div className="flex items-center gap-1">
                                <MapPin className="h-4 w-4 text-slate-500" />
                                <span className="text-slate-500 text-sm">Location:</span>
                                <span className="text-sm ml-1">{dao.address}, {dao.town}, {dao.zip}</span>
                            </div>

                            <div className="flex items-center gap-1">
                                <Users className="h-4 w-4 text-slate-500" />
                                <span className="text-slate-500 text-sm">People:</span>
                                <span className="text-sm ml-1">{dao.board.length} board, {dao.members.length} members</span>
                            </div>

                            <div className="flex items-center gap-1">
                                <Calendar className="h-4 w-4 text-slate-500" />
                                <span className="text-slate-500 text-sm">Created:</span>
                                <span className="text-sm ml-1">{new Date(dao.createdAt.getTime()).toLocaleDateString()}</span>
                            </div>
                        </div>
                    </div>

                    {/* Purpose - 2 lines with expand option */}
                    <div className="bg-white p-4 rounded-lg shadow-sm">
                        <div className="flex items-center justify-between mb-1">
                            <p className="text-slate-500 text-sm flex items-center">
                                <FileText className="h-4 w-4 mr-1" />
                                Purpose
                            </p>
                            {dao.purpose && dao.purpose.length > 100 && (
                                <button
                                    onClick={() => setShowFullPurpose(!showFullPurpose)}
                                    className="text-xs text-slate-500 flex items-center"
                                >
                                    {showFullPurpose ? 'Show less' : 'Show more'}
                                    <svg
                                        className={`h-4 w-4 ml-1 transform ${showFullPurpose ? 'rotate-180' : ''}`}
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                    >
                                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                                    </svg>
                                </button>
                            )}
                        </div>
                        <div className={`text-sm ${showFullPurpose ? '' : 'line-clamp-2'}`}>
                            {dao.purpose}
                        </div>
                    </div>
                </div>

                {/* Tabs */}
                <Tabs value={activeTab} onValueChange={setActiveTab} className="mt-6">
                    <TabsList className="mb-8 bg-white p-1 shadow-sm">
                        <TabsTrigger value="members">Members</TabsTrigger>
                        <TabsTrigger value="votations">Votations</TabsTrigger>
                        <TabsTrigger value="publications">Publications</TabsTrigger>
                        <TabsTrigger value="documents">Documents</TabsTrigger>
                    </TabsList>

                    {/* Members Tab */}
                    <TabsContent value="members">
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                            {/* Board Members */}
                            <Card>
                                <CardHeader>
                                    <CardTitle>Board</CardTitle>
                                    <CardDescription>People with administrative roles in the DAO</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    {dao.board.length > 0 ? (
                                        <Table>
                                            <TableHeader>
                                                <TableRow>
                                                    <TableHead>Principal ID</TableHead>
                                                </TableRow>
                                            </TableHeader>
                                            <TableBody>
                                                {
                                                    dao.board.map((member, index) => (
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
                                            <h3 className="font-medium mb-1">No Board Members</h3>
                                            <p>This DAO doesn't have any board members yet.</p>
                                        </div>
                                    )}
                                </CardContent>
                            </Card>

                            {/* Members */}
                            <Card>
                                <CardHeader>
                                    <CardTitle>Members</CardTitle>
                                    <CardDescription>All participating members of the DAO</CardDescription>
                                </CardHeader>
                                <CardContent>
                                    {dao.members.length > 0 ? (
                                        <Table>
                                            <TableHeader>
                                                <TableRow>
                                                    <TableHead>Principal ID</TableHead>
                                                </TableRow>
                                            </TableHeader>
                                            <TableBody>
                                                {
                                                    dao.members.map((member, index) => (
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
                                            <h3 className="font-medium mb-1">No Members</h3>
                                            <p>This DAO doesn't have any members yet.</p>
                                        </div>
                                    )}
                                </CardContent>
                            </Card>
                        </div>
                    </TabsContent>

                    {/* Votations Tab - for future functionality */}
                    <TabsContent value="votations">
                        <Card>
                            <CardHeader>
                                <div className="flex items-center justify-between">
                                    <CardTitle>Votations</CardTitle>
                                    <Button>Create New Votation</Button>
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
                                        <h3 className="font-medium mb-1">No Votations Yet</h3>
                                        <p>Create a new votation to get started.</p>
                                        <Button className="mt-4">Create First Votation</Button>
                                    </div>
                                )}
                            </CardContent>
                        </Card>
                    </TabsContent>

                    {/* Publications Tab - SOGC Publications */}
                    <TabsContent value="publications">
                        <Card>
                            <CardHeader>
                                <CardTitle>SOGC Publications</CardTitle>
                                <CardDescription>Official publications in the Swiss Official Gazette of Commerce</CardDescription>
                            </CardHeader>
                            <CardContent>
                                {publications.length > 0 ? (
                                    <Table>
                                        <TableHeader>
                                            <TableRow>
                                                <TableHead>Date</TableHead>
                                                <TableHead>Publication ID</TableHead>
                                                <TableHead>Status</TableHead>
                                                <TableHead>Actions</TableHead>
                                            </TableRow>
                                        </TableHeader>
                                        <TableBody>
                                            {publications.map((pubDate, index) => {
                                                const date = new Date(pubDate);
                                                return (
                                                    <TableRow key={index}>
                                                        <TableCell>{date.toLocaleDateString()}</TableCell>
                                                        <TableCell className="font-medium">SOGC-{date.getFullYear()}-{index + 1}</TableCell>
                                                        <TableCell>
                                                            <Badge className="bg-green-100 text-green-800">Published</Badge>
                                                        </TableCell>
                                                        <TableCell>
                                                            <Button variant="ghost" size="sm" className="flex items-center gap-1">
                                                                <ExternalLink className="h-4 w-4" />
                                                                View
                                                            </Button>
                                                        </TableCell>
                                                    </TableRow>
                                                );
                                            })}
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
                    </TabsContent>

                    {/* Documents Tab - for future functionality */}
                    <TabsContent value="documents">
                        <Card>
                            <CardHeader>
                                <CardTitle>Documents</CardTitle>
                                <CardDescription>Important DAO documents and files</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <div className="text-center py-8 text-slate-500">
                                    <File className="h-12 w-12 mx-auto mb-4 text-slate-300" />
                                    <h3 className="font-medium mb-1">No Documents Yet</h3>
                                    <p>There are no documents available for this DAO.</p>
                                </div>
                            </CardContent>
                        </Card>
                    </TabsContent>
                </Tabs>
            </MainContainer>
        </>
    );
};

export default DaoDetailsPage;